import type { PropType, VNode } from 'vue';
declare const _default: import("vue").DefineComponent<{
    name: {
        type: StringConstructor;
        required: true;
    };
    lazy: BooleanConstructor;
    props: {
        type: ObjectConstructor;
        default: () => undefined;
    };
    context: {
        type: ObjectConstructor;
        default: () => {};
    };
    scopeId: {
        type: PropType<string | undefined | null>;
        default: () => undefined;
    };
    source: {
        type: StringConstructor;
        default: () => undefined;
    };
    dangerouslyLoadClientComponents: {
        type: BooleanConstructor;
        default: boolean;
    };
}, (_ctx: any, _cache: any) => (VNode<import("vue").RendererNode, import("vue").RendererElement, {
    [key: string]: any;
}> | VNode<import("vue").RendererNode, import("vue").RendererElement, {
    [key: string]: any;
}>[])[] | VNode<any, any, {
    [key: string]: any;
}>[], unknown, {}, {}, import("vue").ComponentOptionsMixin, import("vue").ComponentOptionsMixin, "error"[], "error", import("vue").PublicProps, Readonly<import("vue").ExtractPropTypes<{
    name: {
        type: StringConstructor;
        required: true;
    };
    lazy: BooleanConstructor;
    props: {
        type: ObjectConstructor;
        default: () => undefined;
    };
    context: {
        type: ObjectConstructor;
        default: () => {};
    };
    scopeId: {
        type: PropType<string | undefined | null>;
        default: () => undefined;
    };
    source: {
        type: StringConstructor;
        default: () => undefined;
    };
    dangerouslyLoadClientComponents: {
        type: BooleanConstructor;
        default: boolean;
    };
}>> & {
    onError?: ((...args: any[]) => any) | undefined;
}, {
    props: Record<string, any>;
    lazy: boolean;
    source: string;
    scopeId: string | null | undefined;
    context: Record<string, any>;
    dangerouslyLoadClientComponents: boolean;
}, {}>;
export default _default;
