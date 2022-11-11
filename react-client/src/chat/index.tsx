import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import { useGetUsersQuery, useLoggedInUserQuery, useRegisterMutation } from "./gql-types";
import { FormEvent, useRef } from "react";

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
    let { data, loading, error, refetch } = useLoggedInUserQuery();

    if (error !== undefined) {
        return <> {error.message}</>;
    }
    if (loading || data === undefined) {
        return <>Loading...</>;
    }

    if (data.loggedInUser === null || data.loggedInUser === undefined) {
        return <Register refetch={refetch} />;
    }

    return (
        <>
            <>Hello {data.loggedInUser.name}!</>
            <div>
                Users:
                <Users />
            </div>
        </>
    );
}

function Register(props: { refetch: () => void }) {
    const input = useRef<HTMLInputElement>(null);
    let [register, { data, loading, error }] = useRegisterMutation();

    if (data) {
        props.refetch();
        return <>Hello {data.register.name}!</>;
    }
    if (loading) {
        return <>...</>;
    }
    if (error) {
        return <>Ohoh: {error.message}</>;
    }

    let onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const name = input.current?.value;

        if (name === undefined) {
            return;
        }
        register({
            variables: {
                name,
            },
        });
    };

    return (
        <form onSubmit={onSubmit}>
            <input ref={input} type="text" />
            <button type={"submit"}>Ok</button>
        </form>
    );
}

function Users() {
    let queryResult = useGetUsersQuery();

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
