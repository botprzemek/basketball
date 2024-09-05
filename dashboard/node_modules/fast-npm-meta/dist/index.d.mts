interface PackageManifest {
    name: string;
    distTags: Record<string, string> & {
        latest: string;
    };
    versions: string[];
    lastSynced: number;
}
interface ResolvedPackageVersion {
    name: string;
    version: string | null;
    specifier: string;
    lastSynced: string;
}

interface ApiOptions {
    force?: boolean;
    apiEndpoint?: string;
    /**
     * Fetch function
     */
    fetch?: typeof fetch;
}
declare const defaultOptions: {
    /**
     * API endpoint for fetching package versions
     *
     * @default 'https://npm.antfu.dev/'
     */
    apiEndpoint: string;
};
declare function getLatestVersionBatch(packages: string[], options?: ApiOptions): Promise<ResolvedPackageVersion[]>;
declare function getLatestVersion(name: string, options?: ApiOptions): Promise<ResolvedPackageVersion>;
declare function getVersionsBatch(packages: string[], options?: ApiOptions): Promise<PackageManifest[]>;
declare function getVersions(name: string, options?: ApiOptions): Promise<PackageManifest>;

declare const NPM_REGISTRY = "https://registry.npmjs.org/";
/**
 * Lightweight replacement of `npm-registry-fetch` function `pickRegistry`'
 *
 * @param scope - scope of package, get from 'npm-package-arg'
 * @param npmConfigs - npm configs, read from `.npmrc` file
 * @param defaultRegistry - default registry, default to 'https://registry.npmjs.org/'
 */
declare function pickRegistry(scope: string | null | undefined, npmConfigs: Record<string, unknown>, defaultRegistry?: string): string;

export { type ApiOptions, NPM_REGISTRY, type PackageManifest, type ResolvedPackageVersion, defaultOptions, getLatestVersion, getLatestVersionBatch, getVersions, getVersionsBatch, pickRegistry };
