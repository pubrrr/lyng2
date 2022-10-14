import { ApolloClient, ApolloProvider, gql, InMemoryCache, useQuery } from "@apollo/client";
import { Query } from "./gql-types";

function ChatApp() {
    const client = new ApolloClient({
        uri: "api/chat/",
        cache: new InMemoryCache(),
    });

    return (
        <ApolloProvider client={client}>
            Users:
            <Users />
        </ApolloProvider>
    );
}

function Users() {
    let queryResult = useQuery<Query>(
        gql`
            {
                getUsers
            }
        `
    );

    if (queryResult.error) {
        return <>Error: {queryResult.error.message}</>;
    }
    if (queryResult.loading) {
        return <>...</>;
    }
    return (
        <ul>
            {queryResult.data?.getUsers.map((user) => (
                <li key={user}>{user}</li>
            ))}
        </ul>
    );
}

export { ChatApp };
