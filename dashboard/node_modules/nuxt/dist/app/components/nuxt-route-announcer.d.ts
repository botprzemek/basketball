import type { Politeness } from '#app/composables/route-announcer';
declare const _default: import("vue").DefineComponent<{
    atomic: {
        type: BooleanConstructor;
        default: boolean;
    };
    politeness: {
        type: () => Politeness;
        default: string;
    };
}, () => import("vue").VNode<import("vue").RendererNode, import("vue").RendererElement, {
    [key: string]: any;
}>, unknown, {}, {}, import("vue").ComponentOptionsMixin, import("vue").ComponentOptionsMixin, {}, string, import("vue").PublicProps, Readonly<import("vue").ExtractPropTypes<{
    atomic: {
        type: BooleanConstructor;
        default: boolean;
    };
    politeness: {
        type: () => Politeness;
        default: string;
    };
}>>, {
    politeness: any;
    atomic: boolean;
}, {}>;
export default _default;
