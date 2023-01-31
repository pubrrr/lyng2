import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import { createClient } from "graphql-ws";
import { ApolloClient, DocumentNode, HttpLink, InMemoryCache, split } from "@apollo/client";
import { getMainDefinition } from "@apollo/client/utilities";

const relativeApiUrl = "api/chat/";

export function getApolloClient() {
    const wsLink = new GraphQLWsLink(
        createClient({
            url: "ws://" + window.location.host + "/" + relativeApiUrl,
        })
    );
    const httpLink = new HttpLink({
        uri: relativeApiUrl,
    });

    const link = split(isSubscriptionOperation, wsLink, httpLink);

    return new ApolloClient({
        cache: new InMemoryCache(),
        link,
    });
}

function isSubscriptionOperation({ query }: { query: DocumentNode }) {
    const definition = getMainDefinition(query);
    return definition.kind === "OperationDefinition" && definition.operation === "subscription";
}
