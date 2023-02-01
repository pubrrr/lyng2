import { codegen } from "@graphql-codegen/core";
import { parse } from "graphql";
import * as typescript from "@graphql-codegen/typescript";
import * as typescriptOperations from "@graphql-codegen/typescript-operations";
import * as typescriptReactApollo from "@graphql-codegen/typescript-react-apollo";
import * as fs from "fs";
import { loadDocuments } from "@graphql-tools/load";
import { GraphQLFileLoader } from "@graphql-tools/graphql-file-loader";

function getInput() {
    return new Promise<string>(function (resolve, reject) {
        const stdin = process.stdin;
        let data = "";

        stdin.setEncoding("utf8");
        stdin.on("data", function (chunk: string) {
            data += chunk;
        });

        stdin.on("end", function () {
            resolve(data);
        });

        stdin.on("error", reject);
    });
}

async function generateGqlTypes(schema: string) {
    const loadedDocuments = await loadDocuments(["operations.gql"], {
        loaders: [new GraphQLFileLoader()],
    });

    return codegen({
        filename: "unusedByTsPlugin.ts",
        plugins: [
            {
                typescript: {},
            },
            {
                typescriptOperations: {},
            },
            {
                typescriptReactApollo: {},
            },
        ],
        schema: parse(schema),
        config: [],
        documents: loadedDocuments,
        pluginMap: {
            typescript,
            typescriptOperations,
            typescriptReactApollo,
        },
    });
}

const output = await getInput().then(generateGqlTypes);
const outputFile = "src/chat/gql-types.ts";

fs.writeFile(outputFile, output, (error: any) => {
    if (error) {
        console.log(error);
    }
});
