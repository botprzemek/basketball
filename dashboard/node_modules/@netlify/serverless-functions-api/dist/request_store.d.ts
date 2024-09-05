/// <reference types="node" />
import { AsyncLocalStorage } from 'node:async_hooks';
interface RequestStoreContent {
    cdnLoopHeader: string | null;
}
export declare const requestStore: AsyncLocalStorage<RequestStoreContent>;
export {};
