import React, { useState } from 'react';
import Button from '@material-ui/core/Button';
import Dialog from '@material-ui/core/Dialog';
import DialogActions from '@material-ui/core/DialogActions';
import DialogContent from '@material-ui/core/DialogContent';
import DialogTitle from '@material-ui/core/DialogTitle';
import TextField from '@material-ui/core/TextField';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import { Item } from '@/api';

type Props = {
    open: boolean,
    title: string,
    item: Item,
    ok: (item: Item) => void,
    [key: string]: any,
};

export default function({ open, title, item, ok, cancel }: Props) {
    const [it, setItem] = useState(item);

    return (
        <Dialog
            {...{open}}
            onClose={cancel}
            aria-labelledby="alert-dialog-title"
            aria-describedby="alert-dialog-description"
        >
            <DialogTitle id="alert-dialog-title"> {title} </DialogTitle>
            <DialogContent style={{ width: 550 }}>
                <List>
                    <ListItem>
                        <TextField fullWidth variant="standard" label="名称" value={it.name} onChange={(e) => setItem({ ...it, name: e.target.value })} />
                    </ListItem>
                    <ListItem>
                        <TextField fullWidth variant="filled" label="WebSocket API" value={it.ws} onChange={(e) => setItem({ ...it, ws: e.target.value })} />
                    </ListItem>
                    <ListItem>
                        <TextField fullWidth variant="filled" label="Bind Address" value={it.listen} onChange={(e) => setItem({ ...it, listen: e.target.value })} />
                    </ListItem>
                    <ListItem>
                        <TextField multiline fullWidth variant="outlined" label="备注" value={it.desc} onChange={(e) => setItem({ ...it, desc: e.target.value })} />
                    </ListItem>
                </List>
            </DialogContent>
            <DialogActions>
                <Button onClick={() => ok(it)} color="primary"> 确定 </Button>
                <Button onClick={cancel} color="primary" autoFocus> 取消 </Button>
            </DialogActions>
        </Dialog>
    );
}