import { ApolloProvider } from "@apollo/client";
import {
    useGetNewUsersSubscription,
    useGetUsersQuery,
    useLoggedInUserQuery,
    useRegisterMutation,
} from "./gql-types";
import { FormEvent, useRef } from "react";
import { getApolloClient } from "./apolloClient";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import AppBar from "@mui/material/AppBar";
import {
    Drawer,
    Fab,
    InputAdornment,
    List,
    ListItem,
    ListItemText,
    TextField,
} from "@mui/material";
import { AccountCircle, Send } from "@mui/icons-material";

const drawerWidth = 240;

export function ChatApp() {
    return (
        <ApolloProvider client={getApolloClient()}>
            <Box sx={{ display: "flex" }}>
                <Chat />
            </Box>
        </ApolloProvider>
    );
}

function Chat() {
    return (
        <>
            <AppBar
                position="fixed"
                sx={{ width: `calc(100% - ${drawerWidth}px)`, ml: `${drawerWidth}px` }}
            >
                <Toolbar>
                    <Typography variant="h6" noWrap component="div">
                        Lyng chat
                    </Typography>
                </Toolbar>
            </AppBar>
            <Drawer
                sx={{
                    width: drawerWidth,
                    flexShrink: 0,
                    "& .MuiDrawer-paper": {
                        width: drawerWidth,
                        boxSizing: "border-box",
                    },
                }}
                variant="permanent"
                anchor="left"
            >
                <Toolbar />
                <Users />
            </Drawer>
            <Box component="main" sx={{ flexGrow: 1, bgcolor: "background.default", p: 3 }}>
                <Toolbar />
                <ChatMain />
            </Box>
        </>
    );
}

function ChatMain() {
    let { data, loading, error, refetch } = useLoggedInUserQuery();

    if (error !== undefined) {
        return <Typography paragraph>{error.message}!</Typography>;
    }
    if (loading || data === undefined) {
        return <Typography paragraph>Loading ...</Typography>;
    }

    if (data.loggedInUser === null || data.loggedInUser === undefined) {
        return <Register refetch={refetch} />;
    }

    return <Typography paragraph>Hello {data.loggedInUser.name}!</Typography>;
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
        <Box component="form" onSubmit={onSubmit} sx={{ mt: 1 }}>
            <TextField
                label="Enter your name"
                id="userName"
                inputRef={input}
                InputProps={{
                    startAdornment: (
                        <InputAdornment position="start">
                            <AccountCircle />
                        </InputAdornment>
                    ),
                }}
            />
            <Fab color="primary" type={"submit"}>
                <Send />
            </Fab>
        </Box>
    );
}

function Users() {
    let queryResult = useGetUsersQuery();
    let subscription = useGetNewUsersSubscription();

    let users = queryResult.data?.getUsers || [];
    if (subscription.data?.getNewUsers !== undefined) {
        users.push(subscription.data.getNewUsers);
    }
    return (
        <>
            <Typography variant={"overline"}>Users:</Typography>
            <List>
                {users.map((user) => (
                    <ListItem key={user.id} disablePadding>
                        <ListItemText primary={user.name} />
                    </ListItem>
                ))}
            </List>
        </>
    );
}
