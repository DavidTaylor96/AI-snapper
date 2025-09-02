export interface AppConfig {
    screenshotsDir: string;
    imageFormat: string;
    jpegQuality: number;
    maxImageSizeMb: number;
    apiKey?: string;
    defaultProvider: string;
}
export declare class AppConfig {
    screenshotsDir: string;
    imageFormat: string;
    jpegQuality: number;
    maxImageSizeMb: number;
    apiKey?: string;
    defaultProvider: string;
    constructor(config?: Partial<AppConfig>);
    static load(): Promise<AppConfig>;
    private static toTomlString;
    save(): Promise<void>;
}
//# sourceMappingURL=config.d.ts.map