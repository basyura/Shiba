import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';

const root = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const html = await readFile(join(root, 'ui', 'index.html'), 'utf8');
const script = html.match(/<script>\s*([\s\S]*?platform-darwin[\s\S]*?)\s*<\/script>/)?.[1];

assert.ok(script, 'platform class script should exist in ui/index.html');

function runPlatformScript({ platform, userAgentDataPlatform }) {
    const classes = [];
    const navigator = {
        platform,
        ...(userAgentDataPlatform === undefined
            ? {}
            : {
                  userAgentData: {
                      platform: userAgentDataPlatform,
                  },
              }),
    };
    const document = {
        body: {
            classList: {
                add(name) {
                    classes.push(name);
                },
            },
        },
    };

    vm.runInNewContext(script, { document, navigator });

    return classes;
}

test('macOS platform adds platform-darwin class', () => {
    assert.deepEqual(runPlatformScript({ platform: 'MacIntel' }), ['platform-darwin']);
});

test('Windows platform adds platform-windows class', () => {
    assert.deepEqual(runPlatformScript({ platform: 'Win32' }), ['platform-windows']);
});

test('Linux platform does not add platform class', () => {
    assert.deepEqual(runPlatformScript({ platform: 'Linux x86_64' }), []);
});

test('userAgentData platform is preferred over navigator platform', () => {
    assert.deepEqual(runPlatformScript({ platform: 'Win32', userAgentDataPlatform: 'macOS' }), ['platform-darwin']);
});
