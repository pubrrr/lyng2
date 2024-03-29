import { codegen } from "@graphql-codegen/core";
import { parse } from "graphql";
import * as typescript from "@graphql-codegen/typescript";
import * as typescriptOperations from "@graphql-codegen/typescript-operations";
import * as typescriptReactApollo from "@graphql-codegen/typescript-react-apollo";
import * as fs from "fs";
import { loadDocuments } from "@graphql-tools/load";
import { GraphQLFileLoader } from "@graphql-tools/graphql-file-loader";
import { readFile } from "fs/promises";
import { exec } from "child_process";

const useLocalSchema = !!process.env.DOCKER;

function getInput() {
    return useLocalSchema
        ? readFile("schema.gql").then((buffer) => buffer.toString())
        : new Promise<string>(function (resolve, reject) {
              exec("cd ../server && cargo run -q --bin export_schema", (error, stdout, stderr) => {
                  if (error) {
                      console.log(`error: ${error.message}`);
                      return reject(error);
                  }
                  if (stderr) {
                      console.log(`stderr: ${stderr}`);
                      return reject(stderr);
                  }
                  console.log("generated GQL schema from Rust code");
                  resolve(stdout);
              });
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
        console.log("error when writing gql-types: " + JSON.stringify(error));
    }
});
