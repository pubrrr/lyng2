import { ApolloProvider } from "@apollo/client";
import {
    useGetNewUsersSubscription,
    useGetUsersQuery,
    useLoggedInUserQuery,
    useRegisterMutation,
} from "./gql-types";
import { FormEvent, useRef } from "react";
import { getApolloClient } from "./apolloClient";

export function ChatApp() {
    return (
        <ApolloProvider client={getApolloClient()}>
            <Chat />
        </ApolloProvider>
    );
}

function Chat() {
    let { data, loading, error, refetch } = useLoggedInUserQuery();

    if (error !== undefined) {
        return <>{error.message}</>;
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
    let subscription = useGetNewUsersSubscription();

    if (queryResult.error) {
        return <>Error: {queryResult.error.message}</>;
    }
    if (queryResult.loading) {
        return <>...</>;
    }

    let users = queryResult.data?.getUsers || [];
    if (subscription.data?.getNewUsers !== undefined) {
        users.push(subscription.data.getNewUsers);
    }
    return (
        <>
            <ul>
                {queryResult.data?.getUsers.map((user) => (
                    <li key={user.id}>{user.name}</li>
                ))}
            </ul>
            {subscription.error && <>Subscription error: {subscription.error}</>}
        </>
    );
}
