const kinobi = require("codama");
const anchorIdl = require("@codama/nodes-from-anchor");
const path = require("path");
const renderers = require('@codama/renderers');

// Paths.
const projectRoot = path.join(__dirname, "..");

const idlDir = path.join(projectRoot, "idl");

const rustClientsDir = path.join(__dirname, "..", "clients", "rust");
// const jsClientsDir = path.join(__dirname, "..", "clients", "js");

// Generate the restaking client in Rust and JavaScript.
const rustRestakingClientDir = path.join(rustClientsDir, "hello_world_ncn_client");
// const jsRestakingClientDir = path.join(jsClientsDir, "hello_world_ncn_client");
const restakingRootNode = anchorIdl.rootNodeFromAnchor(require(path.join(idlDir, "hello_world_ncn.json")));
const restakingKinobi = kinobi.createFromRoot(restakingRootNode);
restakingKinobi.update(kinobi.bottomUpTransformerVisitor([
    {
        // PodU64 -> u64
        select: (nodes) => {
            for (let i = 0; i < nodes.length; i++) {
                if (
                    kinobi.isNode(nodes[i], "structFieldTypeNode") &&
                    nodes[i].type.name === "podU64"
                ) {
                    return true;
                };
            }

            return false;
        },
        transform: (node) => {
            kinobi.assertIsNode(node, ["structFieldTypeNode", "definedTypeLinkNode"]);
            return {
                ...node,
                type: kinobi.numberTypeNode("u64"),
            };
        },
    },
    {
        // PodU32 -> u32
        select: (nodes) => {
            for (let i = 0; i < nodes.length; i++) {
                if (
                    kinobi.isNode(nodes[i], "structFieldTypeNode") &&
                    nodes[i].type.name === "podU32"
                ) {
                    return true;
                }
            }

            return false;
        },
        transform: (node) => {
            kinobi.assertIsNode(node, ["structFieldTypeNode", "definedTypeLinkNode"]);
            return {
                ...node,
                type: kinobi.numberTypeNode("u32"),
            };
        },
    },
    {
        // PodU16 -> u16
        select: (nodes) => {
            for (let i = 0; i < nodes.length; i++) {
                if (
                    kinobi.isNode(nodes[i], "structFieldTypeNode") &&
                    nodes[i].type.name === "podU16"
                ) {
                    return true;
                }
            }

            return false;
        },
        transform: (node) => {
            kinobi.assertIsNode(node, ["structFieldTypeNode", "definedTypeLinkNode"]);
            return {
                ...node,
                type: kinobi.numberTypeNode("u16"),
            };
        },
    },
    // add 8 byte discriminator to accountNode
    {
        select: '[accountNode]',
        transform: (node) => {
            kinobi.assertIsNode(node, "accountNode");
            return {
                ...node,
                data: {
                    ...node.data,
                    fields: [
                        kinobi.structFieldTypeNode({ name: 'discriminator', type: kinobi.numberTypeNode('u64') }),
                        ...node.data.fields
                    ]
                }
            };
        },
    },
]));
restakingKinobi.accept(renderers.renderRustVisitor(path.join(rustRestakingClientDir, "src", "generated"), {
    formatCode: true,
    crateFolder: rustRestakingClientDir,
    deleteFolderBeforeRendering: true,
    toolchain: "+nightly-2024-07-25"
}));
// restakingKinobi.accept(renderers.renderJavaScriptVisitor(path.join(jsRestakingClientDir), {}));

