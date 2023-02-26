import { useGetNewUsersSubscription, useGetUsersQuery, User } from './gql-types';
import Typography from '@mui/material/Typography';
import { Box, List, ListItem, ListItemText } from '@mui/material';
import { useState } from 'react';

function useUsers() {
    const [users, setUsers] = useState<User[]>([]);

    useGetUsersQuery({
        onCompleted: (data) => setUsers((users) => [...users, ...data.getUsers]),
        onError: (error) => console.log(error),
    });

    useGetNewUsersSubscription({
        onData: (options) => {
            const newUser = options.data.data?.getNewUsers;
            if (newUser !== undefined) {
                setUsers((users) => [...users, newUser]);
            } else if (options.data.error !== undefined) {
                console.log(options.data.error);
            }
        },
    });

    return users;
}

export function Users() {
    const users = useUsers();

    return (
        <Box sx={{ p: 2 }}>
            <Typography variant={'overline'}>Users:</Typography>
            <List>
                {users.map((user) => (
                    <ListItem key={user.id} disablePadding>
                        <ListItemText primary={user.name} />
                    </ListItem>
                ))}
            </List>
        </Box>
    );
}
