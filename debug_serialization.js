#!/usr/bin/env node

/**
 * Debug script to test frontend serialization
 * Compare with backend Rust serialization
 */

function sortObjectKeys(obj) {
    if (obj === null || typeof obj !== 'object') {
        return obj;
    }

    if (Array.isArray(obj)) {
        return obj.map(sortObjectKeys);
    }

    const sorted = {};
    const keys = Object.keys(obj).sort();

    for (const key of keys) {
        sorted[key] = sortObjectKeys(obj[key]);
    }

    return sorted;
}

function serializePayload(payload) {
    const sortedPayload = sortObjectKeys(payload);
    return JSON.stringify(sortedPayload);
}

// Test cases that should match backend serialization
const testCases = [
    {
        name: "Magic link payload",
        payload: { magiclink: "8ukaMHhcnJJSEePzD5UYaoHgWib1tr8rS6ms73pC985s" }
    },
    {
        name: "Empty object",
        payload: {}
    },
    {
        name: "Login payload",
        payload: {
            email: "me@arkaitz.dev",
            ui_host: "http://localhost:5173",
            next: "/",
            email_lang: "en",
            pub_key: "abc123"
        }
    },
    {
        name: "Nested object",
        payload: {
            z_field: "last",
            a_field: "first",
            nested: {
                z_nested: "nested_last",
                a_nested: "nested_first"
            }
        }
    }
];

console.log("🔍 Frontend JSON Serialization Test");
console.log("=====================================");

testCases.forEach((test, i) => {
    console.log(`\n[${i + 1}] ${test.name}`);
    console.log("Input:", test.payload);
    const serialized = serializePayload(test.payload);
    console.log("Serialized:", serialized);
    console.log("Length:", serialized.length);
});