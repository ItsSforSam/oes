#ifndef _ERRNO_H
#define _ERRNO_H
/*
 * Due to us not releasing yet, we can thankfully
 * change our magic values around which are exported to userspace
 * (yay!). Not all of POSIX defined values are defined (yet).
 * It should be placed in a category. The first few are general
 * purpose which may be returned by most syscalls/functions.
 * A -ENOSYS can technically appear infront all syscalls (it shouldn't
 * a few are expected to never have this variant, mut it CAN)
 * 
 * Not all POSIX errno value are expected to be defined, mainly these
 * are values which
 * 1. Are niche enough where an average application doesn't relay on them
 * 2. Not something a kernel should implement (ENOPKG). The more common may
 *    be included for completeness
 * 
 */
 /* @TODO: Remove the above (and this) message before 1.0, and
  * ensure they are consistent with kernel-core's error type
  *        kernel
 */
/*
 * -ENOSYS, when returned from a syscall, indicates that
 * for one reason or another, the requested syscall is
 * not available. Could be using a old version, or just
 * calling syscall with random values (which you shouldn't do)
 */
#define ENOSYS     1           /* Function/syscall not implemented */
#define EINVAL     2          /* Invalid Argument */ 
#define EOPNOTSUPP 3          /* Operation not supported */
#define ENOTSUP    EOPNOTSUPP
#define ENOMEM     4          /* Cannot Allocate Memory */
#define EOVERFLOW  5          /* Buffer is too small for requested data*/
#define ENOBUFS    EOVERFLOW  /* OS has no buffer space available */
#define EAGAIN     6          /* Resource temporarily unavailable, try again */
#define EINTR      7          /* Syscall was interupted, try again */
#define ERESTART   EINTR
#define EBADE      8          /* Invalid exchange */
#define E2BIG      9          /* Argument list is too big*/
// FD Errors
#define EBADF      10         /* Bad File Descriptor */
#define EBADFD     11         /* File Descriptor in bad state */
#define ESTALE     12         /* Stale file handle */
// IO/File System errors
#define EIO        13         /* Input/Output error */
#define EPERM      14         /* Permission denied */
#define EEXIST     15         /* File Exists */
/*
 *  NOTE: These can be deceptive, EMFILE can be returned when ENFILE,
 *        is a more accurate reason, and vice-vera. The exact cause is
 *        intentially unspecified in our case. 
 *        You shouldn't do anything critical depending on the
 *        difference of these two values
 */
#define EMFILE     16         /* Reached max open files */
#define ENFILE     17         /* Reached max open files for system */
#define ETXTBSY    18         /* Text file busy (depends on Filesystem) */
#define EROFS      19         /* Read only filesystem */
#define ENOTDIR    20         /* Not a directory, expected directory */
#define EISDIR     21         /* A directory, didn't expect a directory  */
#define ENOTEMPTY  22         /* Directory not empty */

/*
 * Depending on the File System, a different errno may be returned
 * for ENOSPC and EDQUOT
 * Examples including (but not limited too) is EPERM, EIO, and EROFS.
 * But when the respecting errno is returned, you can be safely sure it's the quota
 * or lack of space
*/
#define EDQUOT     23         /* Quota Reached */
#define ENOSPC     24         /* Out of space */
#define ELOOP      25         /* Too many levels of symbolic links */
#define EMLINK     26         /* Too many (hard) links */
#define ENOEXEC    27         /* File Found, but could not be executed */
#define ELIBEXEC   28         /*  Cannot exec a shared library directly */
#define ELIBACC    29         /* Can not access a needed shared library */
#define ENXIO      30         /* No such device or address */
#define ENODEV     ENXIO
#define ENOTTY     31         /* Inappropriate ioctl for device */

// Super Specific, used by very few syscalls (or for specific edge cases)

#define ENAMETOOLONG    141 /* File name is too long */
#define ENOANO          142 /* No Anode */
#define ENOTRECOVERABLE 143 /* State not recoverable */
#endif /* _ERRNO_H */
