import { ApolloProvider } from '@apollo/client';
import { useLoggedInUserQuery } from './gql-types';
import { getApolloClient } from './apolloClient';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import { Button, CircularProgress, Drawer } from '@mui/material';
import { Users } from './Users';
import { Register } from './Register';
import { PropsWithChildren } from 'react';
import { Chat } from './Chat';
import { ArrowBackIosNew } from '@mui/icons-material';

const drawerWidth = 240;

export function ChatApp() {
    return (
        <ApolloProvider client={getApolloClient()}>
            <Box sx={{ display: 'flex', height: '100vh' }}>
                <ChatContainer />
            </Box>
        </ApolloProvider>
    );
}

function ChatContainer() {
    let { data } = useLoggedInUserQuery();

    let title = 'Lyng Chat';
    if (data?.loggedInUser) {
        title += ' - ' + data.loggedInUser.name;
    }

    return (
        <>
            <Header title={title} />
            <Box sx={{ zIndex: 0 }}>
                <Sidebar>{data?.loggedInUser?.name && <Users />}</Sidebar>
            </Box>
            <Box
                component='main'
                sx={{
                    flexGrow: 1,
                    bgcolor: 'background.default',
                    p: 3,
                    display: 'flex',
                    flexDirection: 'column',
                }}
            >
                <Toolbar />
                <ChatMainContainer />
            </Box>
        </>
    );
}

function Header(props: { title: string }) {
    return (
        <AppBar position='fixed'>
            <Toolbar>
                <Button href='/'>
                    <ArrowBackIosNew sx={{ color: 'white' }} />
                </Button>
                <Box sx={{ position: 'fixed', ml: `${drawerWidth}px` }}>
                    <Typography variant='h6' noWrap component='div'>
                        {props.title}
                    </Typography>
                </Box>
            </Toolbar>
        </AppBar>
    );
}

function Sidebar(props: PropsWithChildren) {
    return (
        <Drawer
            sx={{
                width: drawerWidth,
                flexShrink: 0,
                '& .MuiDrawer-paper': {
                    width: drawerWidth,
                    boxSizing: 'border-box',
                },
            }}
            variant='permanent'
            anchor='left'
        >
            <Toolbar />
            {props.children}
        </Drawer>
    );
}

function ChatMainContainer() {
    let { data, loading, error, refetch } = useLoggedInUserQuery();

    if (error !== undefined) {
        return <Typography paragraph>{error.message}</Typography>;
    }
    if (loading || data === undefined) {
        return <CircularProgress />;
    }

    if (data.loggedInUser === null || data.loggedInUser === undefined) {
        return <Register onSuccess={refetch} />;
    }

    return <Chat />;
}
