import {
    ApolloClient,
    ApolloProvider,
    gql,
    InMemoryCache,
    useMutation,
    useQuery,
} from "@apollo/client";
import { Mutation, Query } from "./gql-types";
import { useRef } from "react";

function ChatApp() {
    const client = new ApolloClient({
        uri: "api/chat/",
        cache: new InMemoryCache(),
    });

    return (
        <ApolloProvider client={client}>
            <Chat />
        </ApolloProvider>
    );
}

function Chat() {
    return (
        <>
            <div>
                <Register />
            </div>
            <div>
                Users:
                <Users />
            </div>
        </>
    );
}

function Register() {
    const input = useRef<HTMLInputElement>(null);
    let [register, { data, loading, error }] = useMutation<Mutation>(gql`
        mutation register($name: String!) {
            register(name: $name) {
                name
            }
        }
    `);

    if (data) {
        return <>Hello {data.register.name}!</>;
    }
    if (loading) {
        return <>...</>;
    }
    if (error) {
        return <>Ohoh: {error.message}</>;
    }

    return (
        <form
            onSubmit={(e) => {
                e.preventDefault();
                register({
                    variables: {
                        name: input.current?.value,
                    },
                });
            }}
        >
            <input ref={input} type="text" />
            <button type={"submit"}>Ok</button>
        </form>
    );
}

function Users() {
    let queryResult = useQuery<Query>(
        gql`
            {
                getUsers {
                    name
                    id
                }
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
                <li key={user.id}>{user.name}</li>
            ))}
        </ul>
    );
}

export { ChatApp };
