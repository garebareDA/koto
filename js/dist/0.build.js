(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../wasm/koto.js":
/*!***********************!*\
  !*** ../wasm/koto.js ***!
  \***********************/
/*! exports provided: run, __wbg_log_eee048adc66327f1, __wbg_instanceof_Window_747b56d25bab9510, __wbg_document_c9bb82e72b87972b, __wbg_getElementById_66a113a03886aac6, __wbg_nodeValue_e25985d41d8bf0bd, __wbg_setnodeValue_a917cb3cefd18d3d, __wbg_newnoargs_714dec97cfe3da72, __wbg_call_652fa4cfce310118, __wbg_globalThis_8f997d48cb67f28e, __wbg_self_8a533577b0c752d3, __wbg_window_5912543aff64b459, __wbg_global_69b29294e4daedff, __wbindgen_is_undefined, __wbindgen_object_clone_ref, __wbindgen_object_drop_ref, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./koto_bg.wasm */ \"../wasm/koto_bg.wasm\");\n/* harmony import */ var _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./koto_bg.js */ \"../wasm/koto_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"run\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_eee048adc66327f1\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_log_eee048adc66327f1\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_Window_747b56d25bab9510\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_instanceof_Window_747b56d25bab9510\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_document_c9bb82e72b87972b\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_document_c9bb82e72b87972b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getElementById_66a113a03886aac6\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getElementById_66a113a03886aac6\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_nodeValue_e25985d41d8bf0bd\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_nodeValue_e25985d41d8bf0bd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_setnodeValue_a917cb3cefd18d3d\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_setnodeValue_a917cb3cefd18d3d\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_714dec97cfe3da72\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newnoargs_714dec97cfe3da72\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_652fa4cfce310118\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_call_652fa4cfce310118\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_8f997d48cb67f28e\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_globalThis_8f997d48cb67f28e\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_8a533577b0c752d3\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_8a533577b0c752d3\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_5912543aff64b459\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_window_5912543aff64b459\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_69b29294e4daedff\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_global_69b29294e4daedff\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_clone_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _koto_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n_koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_start\"]();\n\n\n//# sourceURL=webpack:///../wasm/koto.js?");

/***/ }),

/***/ "../wasm/koto_bg.js":
/*!**************************!*\
  !*** ../wasm/koto_bg.js ***!
  \**************************/
/*! exports provided: run, __wbg_log_eee048adc66327f1, __wbg_instanceof_Window_747b56d25bab9510, __wbg_document_c9bb82e72b87972b, __wbg_getElementById_66a113a03886aac6, __wbg_nodeValue_e25985d41d8bf0bd, __wbg_setnodeValue_a917cb3cefd18d3d, __wbg_newnoargs_714dec97cfe3da72, __wbg_call_652fa4cfce310118, __wbg_globalThis_8f997d48cb67f28e, __wbg_self_8a533577b0c752d3, __wbg_window_5912543aff64b459, __wbg_global_69b29294e4daedff, __wbindgen_is_undefined, __wbindgen_object_clone_ref, __wbindgen_object_drop_ref, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module, global) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return run; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_eee048adc66327f1\", function() { return __wbg_log_eee048adc66327f1; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_Window_747b56d25bab9510\", function() { return __wbg_instanceof_Window_747b56d25bab9510; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_document_c9bb82e72b87972b\", function() { return __wbg_document_c9bb82e72b87972b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getElementById_66a113a03886aac6\", function() { return __wbg_getElementById_66a113a03886aac6; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_nodeValue_e25985d41d8bf0bd\", function() { return __wbg_nodeValue_e25985d41d8bf0bd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_setnodeValue_a917cb3cefd18d3d\", function() { return __wbg_setnodeValue_a917cb3cefd18d3d; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_714dec97cfe3da72\", function() { return __wbg_newnoargs_714dec97cfe3da72; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_652fa4cfce310118\", function() { return __wbg_call_652fa4cfce310118; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_8f997d48cb67f28e\", function() { return __wbg_globalThis_8f997d48cb67f28e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_8a533577b0c752d3\", function() { return __wbg_self_8a533577b0c752d3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_5912543aff64b459\", function() { return __wbg_window_5912543aff64b459; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_69b29294e4daedff\", function() { return __wbg_global_69b29294e4daedff; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return __wbindgen_object_clone_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./koto_bg.wasm */ \"../wasm/koto_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nfunction run() {\n    _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"run\"]();\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n        return cachedTextEncoder.encodeInto(arg, view);\n    }\n    : function (arg, view) {\n        const buf = cachedTextEncoder.encode(arg);\n        view.set(buf);\n        return {\n            read: arg.length,\n            written: buf.length\n        };\n    });\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nconst __wbg_log_eee048adc66327f1 = function (arg0, arg1) {\n    console.log(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbg_instanceof_Window_747b56d25bab9510 = function (arg0) {\n    var ret = getObject(arg0) instanceof Window;\n    return ret;\n};\n\nconst __wbg_document_c9bb82e72b87972b = function (arg0) {\n    var ret = getObject(arg0).document;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __wbg_getElementById_66a113a03886aac6 = function (arg0, arg1, arg2) {\n    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __wbg_nodeValue_e25985d41d8bf0bd = function (arg0, arg1) {\n    var ret = getObject(arg1).nodeValue;\n    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _koto_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_setnodeValue_a917cb3cefd18d3d = function (arg0, arg1, arg2) {\n    getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);\n};\n\nconst __wbg_newnoargs_714dec97cfe3da72 = function (arg0, arg1) {\n    var ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_call_652fa4cfce310118 = handleError(function (arg0, arg1) {\n    var ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n});\n\nconst __wbg_globalThis_8f997d48cb67f28e = handleError(function () {\n    var ret = globalThis.globalThis;\n    return addHeapObject(ret);\n});\n\nconst __wbg_self_8a533577b0c752d3 = handleError(function () {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_window_5912543aff64b459 = handleError(function () {\n    var ret = window.window;\n    return addHeapObject(ret);\n});\n\nconst __wbg_global_69b29294e4daedff = handleError(function () {\n    var ret = global.global;\n    return addHeapObject(ret);\n});\n\nconst __wbindgen_is_undefined = function (arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbindgen_object_clone_ref = function (arg0) {\n    var ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_object_drop_ref = function (arg0) {\n    takeObject(arg0);\n};\n\nconst __wbindgen_throw = function (arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../js/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module), __webpack_require__(/*! ./../js/node_modules/webpack/buildin/global.js */ \"./node_modules/webpack/buildin/global.js\")))\n\n//# sourceURL=webpack:///../wasm/koto_bg.js?");

/***/ }),

/***/ "../wasm/koto_bg.wasm":
/*!****************************!*\
  !*** ../wasm/koto_bg.wasm ***!
  \****************************/
/*! exports provided: memory, run, main, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_exn_store, __wbindgen_start */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./koto_bg.js */ \"../wasm/koto_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../wasm/koto_bg.wasm?");

/***/ }),

/***/ "./node_modules/webpack/buildin/global.js":
/*!***********************************!*\
  !*** (webpack)/buildin/global.js ***!
  \***********************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("var g;\n\n// This works in non-strict mode\ng = (function() {\n\treturn this;\n})();\n\ntry {\n\t// This works if eval is allowed (see CSP)\n\tg = g || new Function(\"return this\")();\n} catch (e) {\n\t// This works if the window reference is available\n\tif (typeof window === \"object\") g = window;\n}\n\n// g can still be undefined, but nothing to do about it...\n// We return undefined, instead of nothing here, so it's\n// easier to handle this case. if(!global) { ...}\n\nmodule.exports = g;\n\n\n//# sourceURL=webpack:///(webpack)/buildin/global.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);