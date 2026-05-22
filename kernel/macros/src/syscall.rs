use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use core::fmt;
use std::result::Result as stdResult;
use syn::{Error, Result, ReturnType, Signature, Token, spanned::Spanned};

pub struct SyscallItem {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub sig: SyscallSignature,
    pub block: Box<syn::Block>
}
#[derive(Clone)]
pub struct SyscallSignature {
    pub async_token: Token![async],
    pub unsafety: Token![unsafe],
    /// The ABI is "C"
    pub abi: syn::Abi,
    pub fn_token: Token![fn],
    pub ident: syn::Ident,
    pub paren_token: syn::token::Paren,
    pub inputs: syn::punctuated::Punctuated<syn::FnArg, Token![,]>,
    pub output: syn::ReturnType,
}
#[derive(Debug)]
pub struct IncorrectSignatureError(Vec<String>);
impl std::fmt::Display for IncorrectSignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0.iter(){
            writeln!(f,"Error: {}",m)?;
        }
        Ok(())
    }
}
impl std::error::Error for IncorrectSignatureError {}

impl std::convert::TryFrom<syn::ItemFn> for SyscallItem {
    type Error = IncorrectSignatureError;

    fn try_from(value: syn::ItemFn) -> stdResult<SyscallItem, IncorrectSignatureError> {
        let mut sig: stdResult<SyscallSignature,_> = value.sig.try_into();
        match value.vis{
            _ if sig.is_err() => {
                match sig{
                    Ok(_) => unreachable!(),
                    Err(ref mut e) => {
                        e.0.push("no visibility for a syscall".to_owned());
                        
                    }
                };
                

            },
            
            syn::Visibility::Inherited => {/* No visibility given */},
            _ => {
                Err(IncorrectSignatureError(vec![String::from("Syscalls should not be visible to rust code")]))?;
            }
        };
        let sig = sig?;

        
        Ok(
            SyscallItem{
                attrs: value.attrs,
                vis: syn::Visibility::Inherited,
                sig,
                block: value.block,
            }
        )
    }
}

impl std::convert::TryFrom<syn::Signature> for SyscallSignature {
    type Error = IncorrectSignatureError;

    fn try_from(value: syn::Signature) -> stdResult<Self, Self::Error> {
        let mut err: Vec<String> = Vec::new();
        if value.variadic.is_some() {
            err.push("Syscall cannot be a variadic function".to_owned());
        }
        if value.unsafety.is_none() {
            err.push("Syscall implementations have a safety requirements, and can be not sound if done wrong. Mark function, as `unsafe` to mark as such".to_owned());
        }
        match value.abi {
            Some(ref abi) => match abi.name {
                Some(ref a) if a.value() != "C" => {
                    use std::fmt::Write;
                    let attempted = a.value();
                    let mut msg = String::new();
                    
                    // This cannot return `Err`, from write, as currently it uses String.push_str(), but can panic
                    // if somehow msg capacity passes iSize::MAX, which shouldn't occur regardless
                    write!(msg, "Syscall function must be marked with `extern \"C\". Found `extern {:?}`",attempted).unwrap();
                    err.push(msg);
                
                }
                Some(ref _c_abi) => {/* Uses extern "C" */}
                None => err.push(
                    "Syscall function must be marked with `extern \"C\"`. Found just `extern`".to_owned(),
                ),
            },
            None => err.push("Syscall functions must be marked `extern \"C\"`. Found no extern, cannot use Rust's ABI".to_owned()),
        }
        
        if value.inputs.iter().any(|i|{
            use syn::FnArg;
            match i{
                FnArg::Receiver(_) => true,
                FnArg::Typed(_) => false
            }
        }){
            err.push("Syscall function cannot be a receiver".to_owned());
        }
        if value.asyncness.is_some(){
            // It isn't going to expand as async, due to how syntactic sugar
            err.push("async functions are not ffi safe and require generic ".to_owned());
        }
        if !err.is_empty() {
            return Err(IncorrectSignatureError(err));
        }
        let s = value.span();
        Ok(SyscallSignature {
            async_token: Token![async](s),
            unsafety: Token![unsafe](s),
            abi: value.abi.unwrap(),
            fn_token: Token![fn](s),
            ident: value.ident,
            paren_token: value.paren_token,
            inputs: value.inputs,
            output: value.output
        }
        )
    }
}

impl syn::parse::Parse for SyscallSignature{
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let og_sig: syn::Signature = input.parse()?;
        std::convert::TryInto::<SyscallSignature>::try_into(og_sig).map_err(|e|{
            syn::Error::new(input.span(), e)
        })
        

    }
}
impl From<SyscallItem> for syn::ItemFn{
    fn from(value: SyscallItem) -> Self {
        syn::ItemFn{
            attrs: value.attrs,
            vis: value.vis,
            sig: value.sig.into(),
            block: value.block,
        }
    }
}
impl From<SyscallSignature> for syn::Signature{
    fn from(value: SyscallSignature) -> Self {
        Signature {
            constness: None,
            asyncness: Some(value.async_token),
            unsafety: Some(value.unsafety),
            abi: Some(value.abi),
            fn_token: value.fn_token,
            ident: value.ident,
            generics: Default::default(),
            paren_token: value.paren_token,
            inputs: value.inputs,
            variadic: None,
            output: value.output,
        }
    }
}
impl syn::parse::Parse for SyscallItem{

    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis:syn::Visibility = input.parse()?;
        let sig:SyscallSignature = input.parse()?;
        todo!()
    }
}
impl quote::ToTokens for SyscallSignature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.abi.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.paren_token.surround(tokens, |f|{
            self.inputs.to_tokens(f);
        });
        self.output.to_tokens(tokens);
    }
}

impl quote::ToTokens for SyscallItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.iter());
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.brace_token.surround(tokens, |token|{
            token.append_all(&self.block.stmts);
        });
    }
}
pub fn syscall_impl(input: SyscallItem) -> Result<TokenStream> {
    let span = input.span();
    let output = &input.sig.output;
    use syn::Type;
    let new_out:ReturnType = match output{
        
        syn::ReturnType::Default => {syn::ReturnType::Default},
        syn::ReturnType::Type(arrow, ty) => {
            match &**ty{
                Type::BareFn(_) | Type::Slice(_) => {
                    Err(Error::new(span, BadReturnType("Cannot pass a rust pointer to Userspace")))?
                    
                },
                Type::Reference(reference) => Err(Error::new(span, BadReturnType("Cannot pass a rust pointer to Userspace. If you need to pass a pointer, use a raw pointer")))?,
                Type::Infer(_) => Err(Error::new(span, BadReturnType("Cannot infer syscall function. This is to prevent breaking userspace")))?,
                Type::Macro(_) => Err(Error::new(span, BadReturnType("No macros as return (How do you even get this?)")))?,
                t => syn::ReturnType::Type(*arrow,Box::new(t.clone())),
            }
        }
    };
    Ok(input.into_token_stream())
}
/// Get type name, if possible
/// 
/// This will return the human readable string, and may 
/// not be valid to parse into [`Ident`]
/// 
/// if type is not able to 
/// 
/// [`Ident`]: syn::Ident
fn get_type_name(type_: &syn::Type) -> Option<String>{
    use syn::Type;
    match type_{
        Type::Reference(b) => {
            Some(get_type_name(&b.elem)?)
        },
        Type::Path(path) => {
            path.path.get_ident().map(|i|i.to_string())

        },
        Type::Array(a) =>{
            let ty = get_type_name(&a.elem)?;
            use syn::Expr;
            let len = match &a.len{
                Expr::Infer(_) => String::from("_"),
                Expr::Lit(v) => get_lit_printable(&v),
                other => unimplemented!("Len expr having {other:?}")
            };
            Some(format!("[{ty};{len}]"))
        },
        _ => None
    }
}
#[expect(unused)]
fn get_lit_printable(expr:&syn::ExprLit) -> String{
    match &expr.lit{
        syn::Lit::Str(lit_str) => lit_str.value(),
        syn::Lit::Int(lit_int) => lit_int.to_string(),
        // We don't use it here, but MAYBE in the future
        // Don't mark with todo!(), to avoid ide 
        other => unimplemented!("Literal type `{other:?}` reached but not implemented"),
    }
}
#[derive(Debug)]
pub struct BadReturnType(&'static str);
impl std::error::Error for BadReturnType {}
impl fmt::Display for BadReturnType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syscall Bad Return Type: {}",self.0)
    }
}