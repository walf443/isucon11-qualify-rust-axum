initSidebarItems({"struct":[["GenericManualResetEvent","A synchronization primitive which can be either in the set or reset state."],["GenericMutex","A futures-aware mutex."],["GenericMutexGuard","An RAII guard returned by the `lock` and `try_lock` methods. When this structure is dropped (falls out of scope), the lock will be unlocked."],["GenericMutexLockFuture","A future which resolves when the target mutex has been successfully acquired."],["GenericSemaphore","A futures-aware semaphore."],["GenericSemaphoreAcquireFuture","A future which resolves when the target semaphore has been successfully acquired."],["GenericSemaphoreReleaser","An RAII guard returned by the `acquire` and `try_acquire` methods."],["GenericSharedSemaphore","A futures-aware shared semaphore."],["GenericSharedSemaphoreAcquireFuture","A future which resolves when the target semaphore has been successfully acquired."],["GenericSharedSemaphoreReleaser","An RAII guard returned by the `acquire` and `try_acquire` methods."],["GenericWaitForEventFuture","A Future that is resolved once the corresponding ManualResetEvent has been set"]],"type":[["LocalManualResetEvent","A [`GenericManualResetEvent`] which is not thread-safe."],["LocalMutex","A [`GenericMutex`] which is not thread-safe."],["LocalMutexGuard","A [`GenericMutexGuard`] for [`LocalMutex`]."],["LocalMutexLockFuture","A [`GenericMutexLockFuture`] for [`LocalMutex`]."],["LocalSemaphore","A [`GenericSemaphore`] which is not thread-safe."],["LocalSemaphoreAcquireFuture","A [`GenericSemaphoreAcquireFuture`] for [`LocalSemaphore`]."],["LocalSemaphoreReleaser","A [`GenericSemaphoreReleaser`] for [`LocalSemaphore`]."],["LocalWaitForEventFuture","A [`GenericWaitForEventFuture`] for [`LocalManualResetEvent`]."],["ManualResetEvent","A [`GenericManualResetEvent`] implementation backed by [`parking_lot`]."],["Mutex","A [`GenericMutex`] backed by [`parking_lot`]."],["MutexGuard","A [`GenericMutexGuard`] for [`Mutex`]."],["MutexLockFuture","A [`GenericMutexLockFuture`] for [`Mutex`]."],["Semaphore","A [`GenericSemaphore`] backed by [`parking_lot`]."],["SemaphoreAcquireFuture","A [`GenericSemaphoreAcquireFuture`] for [`Semaphore`]."],["SemaphoreReleaser","A [`GenericSemaphoreReleaser`] for [`Semaphore`]."],["SharedSemaphore","A [`GenericSharedSemaphore`] backed by [`parking_lot`]."],["SharedSemaphoreAcquireFuture","A [`GenericSharedSemaphoreAcquireFuture`] for [`SharedSemaphore`]."],["SharedSemaphoreReleaser","A [`GenericSharedSemaphoreReleaser`] for [`SharedSemaphore`]."],["WaitForEventFuture","A [`GenericWaitForEventFuture`] for [`ManualResetEvent`]."]]});