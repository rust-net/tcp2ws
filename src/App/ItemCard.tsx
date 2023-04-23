import React, { useEffect, useState } from 'react';
import { makeStyles, Theme, createStyles } from '@material-ui/core/styles';
import clsx from 'clsx';
import Card from '@material-ui/core/Card';
import CardHeader from '@material-ui/core/CardHeader';
import CardContent from '@material-ui/core/CardContent';
import CardActions from '@material-ui/core/CardActions';
import Avatar from '@material-ui/core/Avatar';
import IconButton from '@material-ui/core/IconButton';
import Typography from '@material-ui/core/Typography';
import { red, cyan } from '@material-ui/core/colors';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import Box from '@material-ui/core/Box';
import CloudDoneIcon from '@material-ui/icons/CloudDone';
import CloudOffIcon from '@material-ui/icons/CloudOff';
import PlayCircleFilledWhiteIcon from '@material-ui/icons/PlayCircleFilledWhite';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
import api, { Item } from '@/api';
import Snackbar from '@material-ui/core/Snackbar';
import Alert from '@material-ui/lab/Alert';
import Menu from '@material-ui/core/Menu';
import MenuItem from '@material-ui/core/MenuItem';
import EditIcon from '@material-ui/icons/Edit';
import DeleteForeverIcon from '@material-ui/icons/DeleteForever';
import { Dialog, DialogTitle, DialogContent, DialogContentText, DialogActions, Button } from '@material-ui/core';
import { LoadApp } from '..';
import ItemDialog from './ItemDialog';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      background: cyan[50],
      maxWidth: 345,
      margin: 20,
    },
    expand: {
      transform: 'rotate(0deg)',
      marginLeft: 'auto',
      transition: theme.transitions.create('transform', {
        duration: theme.transitions.duration.shortest,
      }),
      color: 'lightseagreen',
    },
    expandOpen: {
      transform: 'rotate(180deg)',
      color: 'red',
    },
    avatar: {
      backgroundColor: red[500],
    },
    menu: {
      fontSize: 22, color: 'black',
    }
  }),
);

function AlertDialog({ ok, cancel }) {
  return (
      < >
          <Dialog
              open
              onClose={cancel}
              aria-labelledby="alert-dialog-title"
              aria-describedby="alert-dialog-description"
          >
              <DialogTitle id="alert-dialog-title"> 警告 </DialogTitle>
              <DialogContent>
                  <DialogContentText id="alert-dialog-description">
                    你真的要删除该配置吗？
                  </DialogContentText>
              </DialogContent>
              <DialogActions>
                  <Button onClick={ok} color="primary"> 确认 </Button>
                  <Button onClick={cancel} color="primary" autoFocus> 取消 </Button>
              </DialogActions>
          </Dialog>
      </>
  );
}

function MoreButton(props: { item: Item, running: boolean }) {
  const classes = useStyles();
  const root = React.useRef();
  const [open, setOpen] = useState(false);
  const [edit, setEdit] = useState(false);
  const [delet, setDelete] = useState(false);
  const [openMsg, setOpenMsg] = useState(false);
  const [msg, setMsg] = useState({ type: null, text: '' });

  const onEdit = React.useCallback(async (item: Item) => {
    if (api.compareItems(props.item, item)) {
      return setEdit(false);
    }
    if (props.running) {
      try {
        let ok = await api.stop(props.item);
        setMsg({ type: 'success', text: ok });
        setOpenMsg(true);
      } catch(e) {
        setMsg({ type: 'error', text: e });
        setOpenMsg(true);
      }
    }
    try {
      let config = await api.get_config();
      config.item = config.item.map(it => {
        if (!api.compareItems(it, props.item)) {
          if (api.compareItems(it, item)) {
            throw '配置重复！';
          }
          return it;
        }
        return item;
      });
      await api.set_config(config);
      setMsg({ type: 'success', text: '修改成功！' });
      setOpenMsg(true);
      setTimeout(LoadApp, 2000);
      setEdit(false);
    } catch(e) {
      setMsg({ type: 'error', text: `修改失败：${e}` });
      setOpenMsg(true);
    }
  }, []);
  const onDelete = React.useCallback(async () => {
    if (props.running) {
      try {
        let ok = await api.stop(props.item);
        setMsg({ type: 'success', text: ok });
        setOpenMsg(true);
      } catch(e) {
        setMsg({ type: 'error', text: e });
        setOpenMsg(true);
      }
    }
    try {
      let config = await api.get_config();
      config.item = config.item.filter(it => !api.compareItems(it, props.item));
      await api.set_config(config);
      setMsg({ type: 'success', text: '删除成功！' });
      setOpenMsg(true);
      setTimeout(LoadApp, 2000);
      setDelete(false);
    } catch(e) {
      setMsg({ type: 'error', text: `删除失败：${e}` });
      setOpenMsg(true);
    }
  }, [props.running]);

  return (
    <div ref={root}>
      <IconButton onClick={() => setOpen(true)} aria-label="settings"> <MoreVertIcon /> </IconButton>
      <Menu open={open} anchorEl={root.current} onClose={() => setOpen(false)}>
        <MenuItem onClick={() => setEdit(true)}> <EditIcon className={classes.menu} /> 修改 </MenuItem>
        <MenuItem onClick={() => setDelete(true)}> <DeleteForeverIcon className={classes.menu} /> 删除 </MenuItem>
      </Menu>
      
      { delet &&
        <AlertDialog ok={onDelete} cancel={() => setDelete(false)}/>
      }

      {/* <ItemDialog open={edit} title='修改配置' item={props.item} ok={onEdit} cancel={() => setEdit(false)} /> */}
      {/* 期望重新生成组件 */}
      { edit &&
        <ItemDialog open title='修改配置' item={props.item} ok={onEdit} cancel={() => setEdit(false)} />
      }

      <Snackbar open={openMsg} autoHideDuration={2000} onClose={() => setOpenMsg(false)}>
        <Alert variant="filled" severity={msg.type}>{ msg.text }</Alert>
      </Snackbar>
    </div>
  );
}

export default function(props: { item: Item, i: number, running: boolean }) {
  const classes = useStyles();
  const [expanded, setExpanded] = useState(props.running); // 是否正在运行
  const [openMsg, setOpenMsg] = useState(false);
  const [msg, setMsg] = useState({ type: null, text: '' });

  useEffect(() => setExpanded(props.running), [props.running]);

  const handleExpandClick = async () => {
    try {
        let ok = await (expanded ? api.stop : api.start)(props.item);
        setMsg({ type: 'success', text: ok });
        setOpenMsg(true);
        setExpanded(!expanded);
    } catch(e) {
        setMsg({ type: 'error', text: e });
        setOpenMsg(true);
    }
  };

  return (
    <Card className={classes.root}>
      <CardHeader
        avatar={ <Avatar aria-label="recipe" className={classes.avatar}> { props.i } </Avatar> }
        action={ <MoreButton {...props} running={expanded} /> }
        title={props.item.name}
        subheader={props.item.ws}
      />

      <Box padding={5} display="flex" flexDirection="row" flexWrap="wrap" alignItems="flex-start" justifyContent="center">
        <IconButton onClick={handleExpandClick}>
          {expanded ?  <CloudDoneIcon style={{ fontSize: 200, color: 'lightseagreen' }} /> : <CloudOffIcon color='disabled' style={{ fontSize: 200 }} />}
        </IconButton>
        {props.item.listen}
      </Box>

      <CardContent>
        <Typography  variant="body2" color="textSecondary" component="pre">
          {props.item.desc}
        </Typography>
      </CardContent>

      <CardActions disableSpacing>
        <IconButton
          className={clsx(classes.expand, {
            [classes.expandOpen]: expanded,
          })}
          onClick={handleExpandClick}
        >
            {expanded ? <PauseCircleFilledIcon /> : <PlayCircleFilledWhiteIcon />}
        </IconButton>
      </CardActions>

      <Snackbar open={openMsg} autoHideDuration={2000} onClose={() => setOpenMsg(false)}>
        <Alert variant="filled" severity={msg.type}>{ msg.text }</Alert>
      </Snackbar>
    </Card>
  );
}
