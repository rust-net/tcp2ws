import React, { useState } from 'react';
import Button from '@material-ui/core/Button';
import Dialog from '@material-ui/core/Dialog';
import DialogActions from '@material-ui/core/DialogActions';
import DialogContent from '@material-ui/core/DialogContent';
import DialogContentText from '@material-ui/core/DialogContentText';
import DialogTitle from '@material-ui/core/DialogTitle';
import Snackbar from '@material-ui/core/Snackbar';
import Alert from '@material-ui/lab/Alert';

import api from '@/api';

function AlertDialog({ ok, cancel }) {
    return (
        < >
            <Dialog
                open
                onClose={cancel}
                aria-labelledby="alert-dialog-title"
                aria-describedby="alert-dialog-description"
            >
                <DialogTitle id="alert-dialog-title"> 你真的要关闭本程序吗？ </DialogTitle>
                <DialogContent>
                    <DialogContentText id="alert-dialog-description">
                        关闭后再次双击运行.exe文件可重新启动并自动进入本管理页面。
                    </DialogContentText>
                </DialogContent>
                <DialogActions>
                    <Button onClick={ok} color="primary"> 退出 </Button>
                    <Button onClick={cancel} color="primary" autoFocus> 取消 </Button>
                </DialogActions>
            </Dialog>
        </>
    );
}

type ExitButton = React.FC<{ children: (string | React.ReactElement)[] }>;

export default (({ children }) => {

    const [exit, setExit] = useState(false);
    const [exited, setExited] = useState(false);
    const [msg, setMsg] = useState('');
    
    async function onExit() {
        let msg = await api.exit();
        setMsg(msg);
        setExited(true);
    }

    return (
        < >
            <Button onClick={() => setExit(true)} color="inherit">
                { children }
            </Button>
            { !exited && exit && <AlertDialog ok={onExit} cancel={() => setExit(false)}/> }
            <Snackbar open={exited}>
                <Alert variant="filled" severity="success">{ msg }</Alert>
            </Snackbar>
        </>
    );
}) as ExitButton;