import { useGetNewUsersSubscription, useGetUsersQuery } from "./gql-types";
import Typography from "@mui/material/Typography";
import { List, ListItem, ListItemText } from "@mui/material";

export function Users() {
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
