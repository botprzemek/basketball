import type { ComponentOptions, InjectionKey } from 'vue';
export declare const clientOnlySymbol: InjectionKey<boolean>;
declare const _default: import("vue").DefineComponent<Readonly<{
    fallback?: any;
    placeholder?: any;
    fallbackTag?: any;
    placeholderTag?: any;
}>, (props: any) => import("vue").VNode<import("vue").RendererNode, import("vue").RendererElement, {
    [key: string]: any;
}>[] | import("vue").VNode<import("vue").RendererNode, import("vue").RendererElement, {
    [key: string]: any;
}> | undefined, unknown, {}, {}, import("vue").ComponentOptionsMixin, import("vue").ComponentOptionsMixin, {}, string, import("vue").PublicProps, Readonly<import("vue").ExtractPropTypes<Readonly<{
    fallback?: any;
    placeholder?: any;
    fallbackTag?: any;
    placeholderTag?: any;
}>>>, {
    readonly fallback?: any;
    readonly placeholder?: any;
    readonly fallbackTag?: any;
    readonly placeholderTag?: any;
}, {}>;
export default _default;
export declare function createClientOnly<T extends ComponentOptions>(component: T): any;
