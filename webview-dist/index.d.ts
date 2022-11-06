export declare function connect(addr: string): Promise<unknown>;
export declare function connectTimeout(addr: string, timeoutMs: number): Promise<unknown>;
export declare function disconnect(): Promise<unknown>;
export declare function sendAndReceive(request: string): Promise<string>;
