(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../client.js":
/*!********************!*\
  !*** ../client.js ***!
  \********************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function activate_tab(id, idx) {\n  UIkit.tab(document.getElementById(id)).show(idx);\n}\n\nfunction notify(level, content) {\n  UIkit.notification(content, { status: level });\n}\n\nfunction on_event(t, k, v) {\n  window.rustimate.client.on_event(t, k, v);\n}\n\nfunction show_modal(id) {\n  UIkit.modal(document.getElementById(id)).show();\n}\n\nfunction wire_textarea(id) {\n  var el = document.getElementById(id);\n\n  var savedValue = el.value;\n  el.value = '';\n  el.baseScrollHeight = el.scrollHeight;\n  el.value = savedValue;\n\n  el.oninput = function() {\n    var minRows = this.getAttribute('data-min-rows')|0, rows;\n    this.rows = minRows;\n    rows = Math.ceil((this.scrollHeight - this.baseScrollHeight + 48) / 24);\n    this.rows = minRows + rows;\n  };\n  el.oninput();\n}\n\nfunction wire_onbeforeunload(socket) {\n  window.addEventListener('beforeunload', (e) => {\n    socket.close();\n  });\n}\n\nwindow.rustimate = {\n  activate_tab: activate_tab,\n  notify: notify,\n  on_event: on_event,\n  show_modal: show_modal,\n  wire_onbeforeunload: wire_onbeforeunload,\n  wire_textarea: wire_textarea\n};\n\n\n//# sourceURL=webpack:///../client.js?");

/***/ }),

/***/ "../pkg sync recursive":
/*!*******************!*\
  !*** ../pkg sync ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"../pkg sync recursive\";\n\n//# sourceURL=webpack:///../pkg_sync?");

/***/ }),

/***/ "../pkg/rustimate_client.js":
/*!**********************************!*\
  !*** ../pkg/rustimate_client.js ***!
  \**********************************/
/*! exports provided: JsClient, __wbindgen_cb_forget, __wbg_log_0990f4cfd1fb8be5, __wbindgen_object_drop_ref, __wbg_notify_20eeafcd52c4a086, __wbg_showmodal_12e67b356071a972, __wbg_wireonbeforeunload_e00d53163c5d9738, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __widl_instanceof_Window, __widl_instanceof_Blob, __widl_f_set_property_CSSStyleDeclaration, __widl_f_create_element_Document, __widl_f_get_element_by_id_Document, __widl_f_location_Document, __widl_f_set_inner_html_Element, __widl_f_new_FileReader, __widl_f_read_as_array_buffer_FileReader, __widl_f_result_FileReader, __widl_f_set_onload_FileReader, __widl_instanceof_HTMLElement, __widl_f_style_HTMLElement, __widl_instanceof_HTMLInputElement, __widl_f_value_HTMLInputElement, __widl_f_set_value_HTMLInputElement, __widl_f_href_Location, __widl_f_data_MessageEvent, __widl_f_append_child_Node, __widl_f_new_WebSocket, __widl_f_send_with_str_WebSocket, __widl_f_send_with_u8_array_WebSocket, __widl_f_set_onopen_WebSocket, __widl_f_set_onerror_WebSocket, __widl_f_set_onclose_WebSocket, __widl_f_set_onmessage_WebSocket, __wbindgen_string_new, __widl_f_set_binary_type_WebSocket, __widl_f_document_Window, __wbg_call_222be890f6f564bb, __wbindgen_object_clone_ref, __wbg_instanceof_ArrayBuffer_3df027e750cad6cd, __wbg_newnoargs_a9cd98b36c38f53e, __wbg_now_65115a9aef2f5270, __wbg_globalThis_1c2aa6db3ecb073e, __wbg_self_e5cdcdef79894248, __wbg_window_44ec8ac43884a4cf, __wbg_global_c9abcb94a14733fe, __wbindgen_is_undefined, __wbg_buffer_89a8560ab6a3d9c6, __wbg_length_60a719ff58c1bd42, __wbg_new_bd2e1d010adb8a1a, __wbg_set_05a3afda446930ee, __wbg_self_e70540c4956ad879, __wbg_require_9edeecb69c9dc31c, __wbg_crypto_58b0c631995fea92, __wbg_getRandomValues_532ec62d8e780edc, __wbg_getRandomValues_40ceff860009fa55, __wbg_randomFillSync_eabbc18af655bfbe, __wbindgen_string_get, __wbindgen_debug_string, __wbindgen_throw, __wbindgen_memory, __wbindgen_closure_wrapper97, __wbindgen_closure_wrapper91, __wbindgen_closure_wrapper93, __wbindgen_closure_wrapper95 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(global) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"JsClient\", function() { return JsClient; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_cb_forget\", function() { return __wbindgen_cb_forget; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_0990f4cfd1fb8be5\", function() { return __wbg_log_0990f4cfd1fb8be5; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_notify_20eeafcd52c4a086\", function() { return __wbg_notify_20eeafcd52c4a086; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_showmodal_12e67b356071a972\", function() { return __wbg_showmodal_12e67b356071a972; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_wireonbeforeunload_e00d53163c5d9738\", function() { return __wbg_wireonbeforeunload_e00d53163c5d9738; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_Window\", function() { return __widl_instanceof_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_Blob\", function() { return __widl_instanceof_Blob; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_property_CSSStyleDeclaration\", function() { return __widl_f_set_property_CSSStyleDeclaration; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_create_element_Document\", function() { return __widl_f_create_element_Document; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_get_element_by_id_Document\", function() { return __widl_f_get_element_by_id_Document; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_location_Document\", function() { return __widl_f_location_Document; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_inner_html_Element\", function() { return __widl_f_set_inner_html_Element; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_new_FileReader\", function() { return __widl_f_new_FileReader; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_read_as_array_buffer_FileReader\", function() { return __widl_f_read_as_array_buffer_FileReader; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_result_FileReader\", function() { return __widl_f_result_FileReader; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_onload_FileReader\", function() { return __widl_f_set_onload_FileReader; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_HTMLElement\", function() { return __widl_instanceof_HTMLElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_style_HTMLElement\", function() { return __widl_f_style_HTMLElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_HTMLInputElement\", function() { return __widl_instanceof_HTMLInputElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_value_HTMLInputElement\", function() { return __widl_f_value_HTMLInputElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_value_HTMLInputElement\", function() { return __widl_f_set_value_HTMLInputElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_href_Location\", function() { return __widl_f_href_Location; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_data_MessageEvent\", function() { return __widl_f_data_MessageEvent; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_append_child_Node\", function() { return __widl_f_append_child_Node; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_new_WebSocket\", function() { return __widl_f_new_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_send_with_str_WebSocket\", function() { return __widl_f_send_with_str_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_send_with_u8_array_WebSocket\", function() { return __widl_f_send_with_u8_array_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_onopen_WebSocket\", function() { return __widl_f_set_onopen_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_onerror_WebSocket\", function() { return __widl_f_set_onerror_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_onclose_WebSocket\", function() { return __widl_f_set_onclose_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_onmessage_WebSocket\", function() { return __widl_f_set_onmessage_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_binary_type_WebSocket\", function() { return __widl_f_set_binary_type_WebSocket; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_document_Window\", function() { return __widl_f_document_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_222be890f6f564bb\", function() { return __wbg_call_222be890f6f564bb; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return __wbindgen_object_clone_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_ArrayBuffer_3df027e750cad6cd\", function() { return __wbg_instanceof_ArrayBuffer_3df027e750cad6cd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_a9cd98b36c38f53e\", function() { return __wbg_newnoargs_a9cd98b36c38f53e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_now_65115a9aef2f5270\", function() { return __wbg_now_65115a9aef2f5270; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_1c2aa6db3ecb073e\", function() { return __wbg_globalThis_1c2aa6db3ecb073e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_e5cdcdef79894248\", function() { return __wbg_self_e5cdcdef79894248; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_44ec8ac43884a4cf\", function() { return __wbg_window_44ec8ac43884a4cf; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_c9abcb94a14733fe\", function() { return __wbg_global_c9abcb94a14733fe; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_89a8560ab6a3d9c6\", function() { return __wbg_buffer_89a8560ab6a3d9c6; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_60a719ff58c1bd42\", function() { return __wbg_length_60a719ff58c1bd42; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_bd2e1d010adb8a1a\", function() { return __wbg_new_bd2e1d010adb8a1a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_05a3afda446930ee\", function() { return __wbg_set_05a3afda446930ee; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_e70540c4956ad879\", function() { return __wbg_self_e70540c4956ad879; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_9edeecb69c9dc31c\", function() { return __wbg_require_9edeecb69c9dc31c; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_58b0c631995fea92\", function() { return __wbg_crypto_58b0c631995fea92; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_532ec62d8e780edc\", function() { return __wbg_getRandomValues_532ec62d8e780edc; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_40ceff860009fa55\", function() { return __wbg_getRandomValues_40ceff860009fa55; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_eabbc18af655bfbe\", function() { return __wbg_randomFillSync_eabbc18af655bfbe; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_get\", function() { return __wbindgen_string_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_debug_string\", function() { return __wbindgen_debug_string; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return __wbindgen_memory; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_closure_wrapper97\", function() { return __wbindgen_closure_wrapper97; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_closure_wrapper91\", function() { return __wbindgen_closure_wrapper91; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_closure_wrapper93\", function() { return __wbindgen_closure_wrapper93; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_closure_wrapper95\", function() { return __wbindgen_closure_wrapper95; });\n/* harmony import */ var _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rustimate_client_bg.wasm */ \"../pkg/rustimate_client_bg.wasm\");\n\n\nconst heap = new Array(32);\n\nheap.fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachedTextEncoder = new TextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nfunction debugString(val) {\n    // primitive types\n    const type = typeof val;\n    if (type == 'number' || type == 'boolean' || val == null) {\n        return  `${val}`;\n    }\n    if (type == 'string') {\n        return `\"${val}\"`;\n    }\n    if (type == 'symbol') {\n        const description = val.description;\n        if (description == null) {\n            return 'Symbol';\n        } else {\n            return `Symbol(${description})`;\n        }\n    }\n    if (type == 'function') {\n        const name = val.name;\n        if (typeof name == 'string' && name.length > 0) {\n            return `Function(${name})`;\n        } else {\n            return 'Function';\n        }\n    }\n    // objects\n    if (Array.isArray(val)) {\n        const length = val.length;\n        let debug = '[';\n        if (length > 0) {\n            debug += debugString(val[0]);\n        }\n        for(let i = 1; i < length; i++) {\n            debug += ', ' + debugString(val[i]);\n        }\n        debug += ']';\n        return debug;\n    }\n    // Test for built-in\n    const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));\n    let className;\n    if (builtInMatches.length > 1) {\n        className = builtInMatches[1];\n    } else {\n        // Failed to match the standard '[object ClassName]'\n        return toString.call(val);\n    }\n    if (className == 'Object') {\n        // we're a user defined class or Object\n        // JSON.stringify avoids problems with cycles, and is generally much\n        // easier than looping through ownProperties of `val`.\n        try {\n            return 'Object(' + JSON.stringify(val) + ')';\n        } catch (_) {\n            return 'Object';\n        }\n    }\n    // errors\n    if (val instanceof Error) {\n        return `${val.name}: ${val.message}\\n${val.stack}`;\n    }\n    // TODO we could test for more things here, like `Set`s and `Map`s.\n    return className;\n}\nfunction __wbg_adapter_20(arg0, arg1, arg2) {\n    _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h271c40debdfb8059\"](arg0, arg1, addHeapObject(arg2));\n}\n\nfunction __wbg_adapter_23(arg0, arg1, arg2) {\n    _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h271c40debdfb8059\"](arg0, arg1, addHeapObject(arg2));\n}\n\nfunction __wbg_adapter_26(arg0, arg1) {\n    _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"_dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcaf6d4770841817e\"](arg0, arg1);\n}\n\nfunction __wbg_adapter_29(arg0, arg1, arg2) {\n    _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h271c40debdfb8059\"](arg0, arg1, addHeapObject(arg2));\n}\n\nfunction handleError(e) {\n    _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n*/\nclass JsClient {\n\n    static __wrap(ptr) {\n        const obj = Object.create(JsClient.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_jsclient_free\"](ptr);\n    }\n    /**\n    * @returns {JsClient}\n    */\n    constructor() {\n        var ret = _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"jsclient_new\"]();\n        return JsClient.__wrap(ret);\n    }\n    /**\n    * @param {string} t\n    * @param {string} k\n    * @param {string} v\n    */\n    on_event(t, k, v) {\n        var ptr0 = passStringToWasm0(t, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        var ptr1 = passStringToWasm0(k, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len1 = WASM_VECTOR_LEN;\n        var ptr2 = passStringToWasm0(v, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len2 = WASM_VECTOR_LEN;\n        _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"jsclient_on_event\"](this.ptr, ptr0, len0, ptr1, len1, ptr2, len2);\n    }\n}\n\nconst __wbindgen_cb_forget = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_log_0990f4cfd1fb8be5 = function(arg0, arg1, arg2, arg3) {\n    console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_notify_20eeafcd52c4a086 = function(arg0, arg1, arg2, arg3) {\n    rustimate.notify(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));\n};\n\nconst __wbg_showmodal_12e67b356071a972 = function(arg0, arg1) {\n    rustimate.show_modal(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbg_wireonbeforeunload_e00d53163c5d9738 = function(arg0) {\n    rustimate.wire_onbeforeunload(getObject(arg0));\n};\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __widl_instanceof_Window = function(arg0) {\n    var ret = getObject(arg0) instanceof Window;\n    return ret;\n};\n\nconst __widl_instanceof_Blob = function(arg0) {\n    var ret = getObject(arg0) instanceof Blob;\n    return ret;\n};\n\nconst __widl_f_set_property_CSSStyleDeclaration = function(arg0, arg1, arg2, arg3, arg4) {\n    try {\n        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_create_element_Document = function(arg0, arg1, arg2) {\n    try {\n        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __widl_f_location_Document = function(arg0) {\n    var ret = getObject(arg0).location;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __widl_f_set_inner_html_Element = function(arg0, arg1, arg2) {\n    getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);\n};\n\nconst __widl_f_new_FileReader = function() {\n    try {\n        var ret = new FileReader();\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_read_as_array_buffer_FileReader = function(arg0, arg1) {\n    try {\n        getObject(arg0).readAsArrayBuffer(getObject(arg1));\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_result_FileReader = function(arg0) {\n    try {\n        var ret = getObject(arg0).result;\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_set_onload_FileReader = function(arg0, arg1) {\n    getObject(arg0).onload = getObject(arg1);\n};\n\nconst __widl_instanceof_HTMLElement = function(arg0) {\n    var ret = getObject(arg0) instanceof HTMLElement;\n    return ret;\n};\n\nconst __widl_f_style_HTMLElement = function(arg0) {\n    var ret = getObject(arg0).style;\n    return addHeapObject(ret);\n};\n\nconst __widl_instanceof_HTMLInputElement = function(arg0) {\n    var ret = getObject(arg0) instanceof HTMLInputElement;\n    return ret;\n};\n\nconst __widl_f_value_HTMLInputElement = function(arg0, arg1) {\n    var ret = getObject(arg1).value;\n    var ptr0 = passStringToWasm0(ret, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {\n    getObject(arg0).value = getStringFromWasm0(arg1, arg2);\n};\n\nconst __widl_f_href_Location = function(arg0, arg1) {\n    try {\n        var ret = getObject(arg1).href;\n        var ptr0 = passStringToWasm0(ret, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        getInt32Memory0()[arg0 / 4 + 1] = len0;\n        getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_data_MessageEvent = function(arg0) {\n    var ret = getObject(arg0).data;\n    return addHeapObject(ret);\n};\n\nconst __widl_f_append_child_Node = function(arg0, arg1) {\n    try {\n        var ret = getObject(arg0).appendChild(getObject(arg1));\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_new_WebSocket = function(arg0, arg1) {\n    try {\n        var ret = new WebSocket(getStringFromWasm0(arg0, arg1));\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_send_with_str_WebSocket = function(arg0, arg1, arg2) {\n    try {\n        getObject(arg0).send(getStringFromWasm0(arg1, arg2));\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_send_with_u8_array_WebSocket = function(arg0, arg1, arg2) {\n    try {\n        getObject(arg0).send(getArrayU8FromWasm0(arg1, arg2));\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __widl_f_set_onopen_WebSocket = function(arg0, arg1) {\n    getObject(arg0).onopen = getObject(arg1);\n};\n\nconst __widl_f_set_onerror_WebSocket = function(arg0, arg1) {\n    getObject(arg0).onerror = getObject(arg1);\n};\n\nconst __widl_f_set_onclose_WebSocket = function(arg0, arg1) {\n    getObject(arg0).onclose = getObject(arg1);\n};\n\nconst __widl_f_set_onmessage_WebSocket = function(arg0, arg1) {\n    getObject(arg0).onmessage = getObject(arg1);\n};\n\nconst __wbindgen_string_new = function(arg0, arg1) {\n    var ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nconst __widl_f_set_binary_type_WebSocket = function(arg0, arg1) {\n    getObject(arg0).binaryType = takeObject(arg1);\n};\n\nconst __widl_f_document_Window = function(arg0) {\n    var ret = getObject(arg0).document;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __wbg_call_222be890f6f564bb = function(arg0, arg1) {\n    try {\n        var ret = getObject(arg0).call(getObject(arg1));\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __wbindgen_object_clone_ref = function(arg0) {\n    var ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nconst __wbg_instanceof_ArrayBuffer_3df027e750cad6cd = function(arg0) {\n    var ret = getObject(arg0) instanceof ArrayBuffer;\n    return ret;\n};\n\nconst __wbg_newnoargs_a9cd98b36c38f53e = function(arg0, arg1) {\n    var ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_now_65115a9aef2f5270 = function() {\n    var ret = Date.now();\n    return ret;\n};\n\nconst __wbg_globalThis_1c2aa6db3ecb073e = function() {\n    try {\n        var ret = globalThis.globalThis;\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __wbg_self_e5cdcdef79894248 = function() {\n    try {\n        var ret = self.self;\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __wbg_window_44ec8ac43884a4cf = function() {\n    try {\n        var ret = window.window;\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __wbg_global_c9abcb94a14733fe = function() {\n    try {\n        var ret = global.global;\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_buffer_89a8560ab6a3d9c6 = function(arg0) {\n    var ret = getObject(arg0).buffer;\n    return addHeapObject(ret);\n};\n\nconst __wbg_length_60a719ff58c1bd42 = function(arg0) {\n    var ret = getObject(arg0).length;\n    return ret;\n};\n\nconst __wbg_new_bd2e1d010adb8a1a = function(arg0) {\n    var ret = new Uint8Array(getObject(arg0));\n    return addHeapObject(ret);\n};\n\nconst __wbg_set_05a3afda446930ee = function(arg0, arg1, arg2) {\n    getObject(arg0).set(getObject(arg1), arg2 >>> 0);\n};\n\nconst __wbg_self_e70540c4956ad879 = function() {\n    try {\n        var ret = self.self;\n        return addHeapObject(ret);\n    } catch (e) {\n        handleError(e)\n    }\n};\n\nconst __wbg_require_9edeecb69c9dc31c = function(arg0, arg1) {\n    var ret = __webpack_require__(\"../pkg sync recursive\")(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_58b0c631995fea92 = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbg_getRandomValues_532ec62d8e780edc = function(arg0) {\n    var ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\nconst __wbg_getRandomValues_40ceff860009fa55 = function(arg0, arg1, arg2) {\n    getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_randomFillSync_eabbc18af655bfbe = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbindgen_string_get = function(arg0, arg1) {\n    const obj = getObject(arg1);\n    var ret = typeof(obj) === 'string' ? obj : undefined;\n    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbindgen_debug_string = function(arg0, arg1) {\n    var ret = debugString(getObject(arg1));\n    var ptr0 = passStringToWasm0(ret, _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbindgen_memory = function() {\n    var ret = _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_closure_wrapper97 = function(arg0, arg1, arg2) {\n\n    const state = { a: arg0, b: arg1, cnt: 1 };\n    const real = (arg0) => {\n        state.cnt++;\n        const a = state.a;\n        state.a = 0;\n        try {\n            return __wbg_adapter_20(a, state.b, arg0);\n        } finally {\n            if (--state.cnt === 0) _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_2\"].get(3)(a, state.b);\n            else state.a = a;\n        }\n    }\n    ;\n    real.original = state;\n    var ret = real;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_closure_wrapper91 = function(arg0, arg1, arg2) {\n\n    const state = { a: arg0, b: arg1, cnt: 1 };\n    const real = (arg0) => {\n        state.cnt++;\n        const a = state.a;\n        state.a = 0;\n        try {\n            return __wbg_adapter_23(a, state.b, arg0);\n        } finally {\n            if (--state.cnt === 0) _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_2\"].get(3)(a, state.b);\n            else state.a = a;\n        }\n    }\n    ;\n    real.original = state;\n    var ret = real;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_closure_wrapper93 = function(arg0, arg1, arg2) {\n\n    const state = { a: arg0, b: arg1, cnt: 1 };\n    const real = (arg0) => {\n        state.cnt++;\n        const a = state.a;\n        state.a = 0;\n        try {\n            return __wbg_adapter_29(a, state.b, arg0);\n        } finally {\n            if (--state.cnt === 0) _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_2\"].get(3)(a, state.b);\n            else state.a = a;\n        }\n    }\n    ;\n    real.original = state;\n    var ret = real;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_closure_wrapper95 = function(arg0, arg1, arg2) {\n\n    const state = { a: arg0, b: arg1, cnt: 1 };\n    const real = () => {\n        state.cnt++;\n        try {\n            return __wbg_adapter_26(state.a, state.b, );\n        } finally {\n            if (--state.cnt === 0) {\n                _rustimate_client_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_2\"].get(3)(state.a, state.b);\n                state.a = 0;\n            }\n        }\n    }\n    ;\n    real.original = state;\n    var ret = real;\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/global.js */ \"./node_modules/webpack/buildin/global.js\")))\n\n//# sourceURL=webpack:///../pkg/rustimate_client.js?");

/***/ }),

/***/ "../pkg/rustimate_client_bg.wasm":
/*!***************************************!*\
  !*** ../pkg/rustimate_client_bg.wasm ***!
  \***************************************/
/*! exports provided: memory, __wbg_jsclient_free, jsclient_new, jsclient_on_event, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_export_2, _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h271c40debdfb8059, _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcaf6d4770841817e, __wbindgen_free, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./rustimate_client.js */ \"../pkg/rustimate_client.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/rustimate_client_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_rustimate_client__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../pkg/rustimate_client */ \"../pkg/rustimate_client.js\");\n/* harmony import */ var _client_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../client.js */ \"../client.js\");\n/* harmony import */ var _client_js__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_client_js__WEBPACK_IMPORTED_MODULE_1__);\n\n\n\n\nwindow.rustimate.client = new _pkg_rustimate_client__WEBPACK_IMPORTED_MODULE_0__[\"JsClient\"]();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/global.js":
/*!***********************************!*\
  !*** (webpack)/buildin/global.js ***!
  \***********************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("var g;\n\n// This works in non-strict mode\ng = (function() {\n\treturn this;\n})();\n\ntry {\n\t// This works if eval is allowed (see CSP)\n\tg = g || new Function(\"return this\")();\n} catch (e) {\n\t// This works if the window reference is available\n\tif (typeof window === \"object\") g = window;\n}\n\n// g can still be undefined, but nothing to do about it...\n// We return undefined, instead of nothing here, so it's\n// easier to handle this case. if(!global) { ...}\n\nmodule.exports = g;\n\n\n//# sourceURL=webpack:///(webpack)/buildin/global.js?");

/***/ })

}]);/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/rustimate_client_bg.wasm": function() {
/******/ 			return {
/******/ 				"./rustimate_client.js": {
/******/ 					"__wbindgen_cb_forget": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_cb_forget"](p0i32);
/******/ 					},
/******/ 					"__wbg_log_0990f4cfd1fb8be5": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_log_0990f4cfd1fb8be5"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_notify_20eeafcd52c4a086": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_notify_20eeafcd52c4a086"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_showmodal_12e67b356071a972": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_showmodal_12e67b356071a972"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_wireonbeforeunload_e00d53163c5d9738": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_wireonbeforeunload_e00d53163c5d9738"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_instanceof_Window": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_instanceof_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_instanceof_Blob": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_instanceof_Blob"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_property_CSSStyleDeclaration": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_property_CSSStyleDeclaration"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__widl_f_create_element_Document": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_create_element_Document"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_get_element_by_id_Document": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_get_element_by_id_Document"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_location_Document": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_location_Document"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_inner_html_Element": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_inner_html_Element"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_new_FileReader": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_new_FileReader"]();
/******/ 					},
/******/ 					"__widl_f_read_as_array_buffer_FileReader": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_read_as_array_buffer_FileReader"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_result_FileReader": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_result_FileReader"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_onload_FileReader": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_onload_FileReader"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_instanceof_HTMLElement": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_instanceof_HTMLElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_style_HTMLElement": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_style_HTMLElement"](p0i32);
/******/ 					},
/******/ 					"__widl_instanceof_HTMLInputElement": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_instanceof_HTMLInputElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_value_HTMLInputElement": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_value_HTMLInputElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_value_HTMLInputElement": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_value_HTMLInputElement"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_href_Location": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_href_Location"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_data_MessageEvent": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_data_MessageEvent"](p0i32);
/******/ 					},
/******/ 					"__widl_f_append_child_Node": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_append_child_Node"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_new_WebSocket": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_new_WebSocket"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_send_with_str_WebSocket": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_send_with_str_WebSocket"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_send_with_u8_array_WebSocket": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_send_with_u8_array_WebSocket"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_set_onopen_WebSocket": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_onopen_WebSocket"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_onerror_WebSocket": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_onerror_WebSocket"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_onclose_WebSocket": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_onclose_WebSocket"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_onmessage_WebSocket": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_onmessage_WebSocket"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_binary_type_WebSocket": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_set_binary_type_WebSocket"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_document_Window": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__widl_f_document_Window"](p0i32);
/******/ 					},
/******/ 					"__wbg_call_222be890f6f564bb": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_call_222be890f6f564bb"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_ArrayBuffer_3df027e750cad6cd": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_instanceof_ArrayBuffer_3df027e750cad6cd"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_a9cd98b36c38f53e": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_newnoargs_a9cd98b36c38f53e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_now_65115a9aef2f5270": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_now_65115a9aef2f5270"]();
/******/ 					},
/******/ 					"__wbg_globalThis_1c2aa6db3ecb073e": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_globalThis_1c2aa6db3ecb073e"]();
/******/ 					},
/******/ 					"__wbg_self_e5cdcdef79894248": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_self_e5cdcdef79894248"]();
/******/ 					},
/******/ 					"__wbg_window_44ec8ac43884a4cf": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_window_44ec8ac43884a4cf"]();
/******/ 					},
/******/ 					"__wbg_global_c9abcb94a14733fe": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_global_c9abcb94a14733fe"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_buffer_89a8560ab6a3d9c6": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_buffer_89a8560ab6a3d9c6"](p0i32);
/******/ 					},
/******/ 					"__wbg_length_60a719ff58c1bd42": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_length_60a719ff58c1bd42"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_bd2e1d010adb8a1a": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_new_bd2e1d010adb8a1a"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_05a3afda446930ee": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_set_05a3afda446930ee"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_self_e70540c4956ad879": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_self_e70540c4956ad879"]();
/******/ 					},
/******/ 					"__wbg_require_9edeecb69c9dc31c": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_require_9edeecb69c9dc31c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_crypto_58b0c631995fea92": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_crypto_58b0c631995fea92"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_532ec62d8e780edc": function(p0i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_getRandomValues_532ec62d8e780edc"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_40ceff860009fa55": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_getRandomValues_40ceff860009fa55"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_eabbc18af655bfbe": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbg_randomFillSync_eabbc18af655bfbe"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper97": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_closure_wrapper97"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper91": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_closure_wrapper91"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper93": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_closure_wrapper93"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper95": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/rustimate_client.js"].exports["__wbindgen_closure_wrapper95"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/rustimate_client_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/rustimate_client_bg.wasm":"rustimate"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "/static/wasm/";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });
