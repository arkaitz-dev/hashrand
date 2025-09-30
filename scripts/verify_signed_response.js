#!/usr/bin/env node

/**
 * Ed25519 Signed Response Verification Utility
 *
 * Validates Ed25519 signatures from HashRand backend responses
 * and extracts payload data for use in bash test scripts.
 */

const { ed25519 } = require('@noble/curves/ed25519.js');
const { bytesToHex, hexToBytes } = require('@noble/hashes/utils.js');

/**
 * Sort object keys recursively for deterministic serialization
 * This must match the backend's serialize_payload_deterministic() function
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

/**
 * Serialize payload deterministically (matching backend)
 */
function serializePayload(payload) {
    const sortedPayload = sortObjectKeys(payload);
    return JSON.stringify(sortedPayload);
}

/**
 * Verify Ed25519 signature of response payload
 */
function verifySignedResponse(responseJson, serverPubKeyHex) {
    try {
        const response = JSON.parse(responseJson);

        // Validate response structure
        if (!response.payload || !response.signature) {
            throw new Error('Invalid response format - missing payload or signature');
        }

        // Determine message to verify based on payload type
        let messageToVerify;
        let serializedPayload;

        // Check if payload is Base64-encoded string (backend response format)
        if (typeof response.payload === 'string' && !response.payload.includes('{')) {
            // Backend sends Base64-encoded payload - use it directly
            messageToVerify = response.payload;
            serializedPayload = response.payload;
        } else {
            // Legacy format: payload is JSON object - serialize it
            serializedPayload = serializePayload(response.payload);
            messageToVerify = serializedPayload;
        }

        // Convert hex strings to bytes
        const publicKeyBytes = hexToBytes(serverPubKeyHex);
        const signatureBytes = hexToBytes(response.signature);
        const messageBytes = new TextEncoder().encode(messageToVerify);

        // Verify Ed25519 signature
        const isValid = ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);

        return {
            valid: isValid,
            payload: response.payload,
            signature: response.signature,
            serializedPayload: serializedPayload
        };
    } catch (error) {
        return {
            valid: false,
            error: error.message,
            payload: null,
            signature: null
        };
    }
}

/**
 * Decode Base64 URL-safe string to JSON object
 */
function decodePayloadBase64(base64String) {
    try {
        // Restore padding and convert to standard Base64
        let base64 = base64String
            .replace(/-/g, '+')
            .replace(/_/g, '/');

        const padding = (4 - (base64.length % 4)) % 4;
        base64 = base64.padEnd(base64.length + padding, '=');

        // Decode Base64 to string
        const jsonString = Buffer.from(base64, 'base64').toString('utf8');

        // Parse JSON
        return JSON.parse(jsonString);
    } catch (error) {
        return null;
    }
}

/**
 * Extract server public key from signed response payload
 */
function extractServerPubKey(responseJson) {
    try {
        const response = JSON.parse(responseJson);

        if (!response.payload) {
            return null;
        }

        // Decode Base64 payload to JSON object
        const payloadObj = decodePayloadBase64(response.payload);

        if (!payloadObj || !payloadObj.server_pub_key) {
            return null;
        }

        return payloadObj.server_pub_key;
    } catch (error) {
        return null;
    }
}

/**
 * Extract specific field from signed response payload
 */
function extractFromPayload(responseJson, fieldName) {
    try {
        const response = JSON.parse(responseJson);

        if (!response.payload) {
            return null;
        }

        // Decode Base64 payload to JSON object
        const payloadObj = decodePayloadBase64(response.payload);

        if (!payloadObj) {
            return null;
        }

        return payloadObj[fieldName] || null;
    } catch (error) {
        return null;
    }
}

// Command line interface
if (require.main === module) {
    const args = process.argv.slice(2);

    if (args.length < 2) {
        console.error('Usage: node verify_signed_response.js <command> <response_json> [server_pub_key]');
        console.error('Commands:');
        console.error('  extract-server-key <response_json>                - Extract server_pub_key from response');
        console.error('  extract-field <response_json> <field_name>        - Extract specific field from payload');
        console.error('  verify <response_json> <server_pub_key>           - Verify signature and return validation result');
        console.error('  extract-verified <response_json> <server_pub_key> <field_name> - Extract field after signature verification');
        process.exit(1);
    }

    const command = args[0];
    const responseJson = args[1];

    try {
        switch (command) {
            case 'extract-server-key':
                const serverKey = extractServerPubKey(responseJson);
                if (serverKey) {
                    console.log(serverKey);
                    process.exit(0);
                } else {
                    console.error('ERROR: server_pub_key not found in response');
                    process.exit(1);
                }
                break;

            case 'extract-field':
                if (args.length < 3) {
                    console.error('ERROR: field_name required');
                    process.exit(1);
                }
                const fieldValue = extractFromPayload(responseJson, args[2]);
                if (fieldValue !== null) {
                    console.log(fieldValue);
                    process.exit(0);
                } else {
                    console.error(`ERROR: field '${args[2]}' not found in payload`);
                    process.exit(1);
                }
                break;

            case 'verify':
                if (args.length < 3) {
                    console.error('ERROR: server_pub_key required');
                    process.exit(1);
                }
                const verifyResult = verifySignedResponse(responseJson, args[2]);
                if (verifyResult.valid) {
                    console.log('VALID');
                    process.exit(0);
                } else {
                    console.error(`ERROR: ${verifyResult.error || 'Invalid signature'}`);
                    process.exit(1);
                }
                break;

            case 'extract-verified':
                if (args.length < 4) {
                    console.error('ERROR: server_pub_key and field_name required');
                    process.exit(1);
                }
                const verifyResult2 = verifySignedResponse(responseJson, args[2]);
                if (verifyResult2.valid) {
                    // Decode Base64 payload to access fields
                    const payloadObj2 = typeof verifyResult2.payload === 'string'
                        ? decodePayloadBase64(verifyResult2.payload)
                        : verifyResult2.payload;

                    if (!payloadObj2) {
                        console.error('ERROR: Failed to decode payload');
                        process.exit(1);
                    }

                    const fieldValue2 = payloadObj2[args[3]];
                    if (fieldValue2 !== undefined) {
                        console.log(fieldValue2);
                        process.exit(0);
                    } else {
                        console.error(`ERROR: field '${args[3]}' not found in verified payload`);
                        process.exit(1);
                    }
                } else {
                    console.error(`ERROR: Signature verification failed - ${verifyResult2.error || 'Invalid signature'}`);
                    process.exit(1);
                }
                break;

            default:
                console.error(`ERROR: Unknown command '${command}'`);
                process.exit(1);
        }
    } catch (error) {
        console.error(`ERROR: ${error.message}`);
        process.exit(1);
    }
}

module.exports = {
    verifySignedResponse,
    extractServerPubKey,
    extractFromPayload,
    sortObjectKeys,
    serializePayload
};