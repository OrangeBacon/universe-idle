; (() => {
    const modules = { ... };

    const loaded = {};
    const paths = [];

    /**
    * @param {string} name - The path to the module to load
    * @returns {any} The loaded module
    */
    function require(name) {
        if (Object.hasOwn(loaded, name)) { return loaded[name] }

        // convert name from relative path to absolute
        let root = [];
        if (paths.length > 0) {
            root.push(...paths[paths.length - 1].split('/'));
            root.pop();
        }
        if (!name.startsWith(".")) {
            root = [];
        }
        root.push(...name.split('/'));
        const absolute = [];
        for (const el of root) {
            if (el === "..") {
                absolute.pop()
            } else if (el !== ".") {
                absolute.push(el);
            }
        }

        const absolute_path = absolute.join('/');

        if (!Object.hasOwn(modules, absolute_path)) {
            throw Error("Unknown module: " + absolute_path + "(reference as: " + name + ")");
        }

        const exports = {};

        paths.push(name);

        // initialise the found module
        modules[absolute_path](exports);

        // store to both the absolute and relative paths to make lookup quicker
        // if require'd again.
        loaded[name] = exports;
        loaded[absolute_path] = exports;

        paths.pop();

        return exports;
    }

    require("main");
})();

