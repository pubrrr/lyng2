import { codegen } from "@graphql-codegen/core";
import { parse } from "graphql";
import * as typescriptPlugin from "@graphql-codegen/typescript";
import * as fs from "fs";

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
    return codegen({
        filename: "unusedByTsPlugin",
        plugins: [
            {
                typescript: {},
            },
        ],
        schema: parse(schema),
        config: [],
        documents: [],
        pluginMap: {
            typescript: typescriptPlugin,
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
