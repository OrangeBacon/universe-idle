declare global {
    namespace JSX {
        type IntrinsicElements = Record<
            keyof HTMLElementTagNameMap,
            Record<string, any>
        >;
    }
}

type JSXComponent = string | typeof Fragment;
type JSXOutput<T> = T extends typeof Fragment ? DocumentFragment : HTMLElement;

// JSX wrapper for create element to make the code nicer to read
export function jsx<T extends JSXComponent>(
    component: T,
    props: Record<string, any> | null,
    _key: any,
): JSXOutput<T> {
    if (!props) props = {};

    if (component == Fragment) {
        const frag = document.createDocumentFragment();

        if (props.children) {
            if (Array.isArray(props.children)) {
                frag.append(...props.children);
            } else {
                frag.append(props.children);
            }
        }

        return frag as JSXOutput<T>;
    }

    const element = document.createElement(component);
    for (const [key, value] of Object.entries(props)) {
        if (key === "children") continue;
        else if (key === "className") element.setAttribute("class", value);
        else element.setAttribute(key, value);
    }

    if (props.children) {
        if (Array.isArray(props.children)) {
            element.append(...props.children);
        } else {
            element.append(props.children);
        }
    }

    return element as JSXOutput<T>;
}

// Don't care about the differences between jsxs and jxs, so forward jsxs to jsx.
export function jsxs(
    component: string | typeof Fragment,
    props: Record<string, any> | null,
    _key: any,
): HTMLElement | DocumentFragment {
    return jsx(component, props, _key);
}

// Force jsx to make a document fragment
export const Fragment = Symbol("Fragment");