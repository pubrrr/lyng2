import { useGetNewUsersSubscription, useGetUsersQuery } from "./gql-types";
import Typography from "@mui/material/Typography";
import { Box, List, ListItem, ListItemText } from "@mui/material";

export function Users() {
    let queryResult = useGetUsersQuery();
    let subscription = useGetNewUsersSubscription();

    let users = queryResult.data?.getUsers || [];
    if (subscription.data?.getNewUsers !== undefined) {
        users.push(subscription.data.getNewUsers);
    } else {
        console.log(subscription.error);
    }

    return (
        <Box sx={{ p: 2 }}>
            <Typography variant={"overline"}>Users:</Typography>
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
