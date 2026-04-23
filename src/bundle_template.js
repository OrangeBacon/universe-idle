; (() => {
    const modules = { ... };

    const loaded = {};

    /**
    * @param {string} name - The path to the module to load
    * @returns {any} The loaded module
    */
    function require(name) {
        if (Object.hasOwn(loaded, name)) { return loaded[name] }

        if (!Object.hasOwn(modules, name)) {
            throw Error("Unknown module: " + name);
        }

        const exports = {};
        modules[name](exports);
        loaded[name] = exports;

        return exports;
    }

    require("main");
})();

