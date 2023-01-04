# Problem Being Solved

This section describes the problem that this pull request aims to solve without making any mention of how the problem is solved or any implementation details. In other words, when filling up this section, you may want to answer the following questions: 

* "Why are you making this pull request?" 
* "What parts of the current behavior are undesirable and why do you wish to change them?"
* "What features are missing and why do you wish to add them? Who are the primary user-group that this feature targets?"

An example of what to include in this section is:

> Currently, when a mutex is locked, it is only unlocked _if_ the operation that it is being locked for succeeds. Otherwise, the mutex lock is not dropped. While this does not pose any immediate issues right now, it could begin to pose problems in the future if we try to recover from `Result::Err`s in code. 
>
> [Here](https://www.github.com/radixdlt/radix-engine-toolkit) is an example of the part of the code that has the issue. As can be seen in the block of code below, the question mark operator is used on the `Result` returned from the `validate_and_modify_data` function which means that if the return is `Result::Ok` then the execution of the block of code continues and the lock is dropped; however, if it is `Result::Err` then the execution of the block is not continued, the error is bubbled up, and the mutex lock is not dropped. 
> ```rust
> let lock_handle = mutex.lock(Lock::ReadWriteLock)?;
> let data = lock_handle.get_mut();
> validate_and_modify_data(data)?; // Issue occurs here
> lock_handle.drop_lock();
> ```

# Solution

Given the context of the problem that you have described above, this section describes the solution that you are proposing to this problem. 

An example of what to include in this section is:

> Given the problem described above, this pull request introduces a higher-level abstraction that is meant to allow for mutexes to be locked, operations to be performed, and then for mutexes to be unlocked regardless of the result of a given operation. This is introduced as a trait called `MutexLockHandler` which is defined as follows:
> ```rust
> trait MutexLockHandler {
>    fn lock_and_handle<F, I, O>(&self, lock: Lock, handler: F) -> Result<O, Error> where F: FnOnce(data: &mut I) -> O;
> }
> ```
> A generic implementation of the above trait is then provided on `Mutex<T>` which handles the locking, handling, and unlocking in a safe manner, the following is what the implementation of the above function looks like:
> ```rust
> fn lock_and_handle<F, I, O>(&self, lock: Lock, handler: F) -> Result<O, Error> where F: FnOnce(data: &mut I) -> O {
>    let lock_handle = mutex.lock(Lock::ReadWriteLock)?;
>    let data = lock_handle.get_mut();
>    let result = handler(data);
>    lock_handle.drop_lock();
>    Ok(result)
> }
> ```

# Recommended Review Path (Optional)

This is an optional section where you can describe the recommended path to follow when reviewing this pull request. This is useful for large pull requests where there are many important changes and many small and inconsequential changes.

An example of what to include in this section is:

> The biggest chunk of changes to the logic were made to:
> * `src/mutex/handling.rs`.
> * `src/mutex/locking.rs`.
> So, the above are the recommended files to review first. 