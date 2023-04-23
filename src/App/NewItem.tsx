import React, { useState } from 'react';
import { makeStyles, Theme, createStyles } from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import CardHeader from '@material-ui/core/CardHeader';
import CardActions from '@material-ui/core/CardActions';
import IconButton from '@material-ui/core/IconButton';
import { red } from '@material-ui/core/colors';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import Box from '@material-ui/core/Box';
import AddIcon from '@material-ui/icons/Add';
import AddCircleIcon from '@material-ui/icons/AddCircle';
import api, { Item } from '@/api';
import Snackbar from '@material-ui/core/Snackbar';
import Alert from '@material-ui/lab/Alert';
import ItemDialog from './ItemDialog';
import { LoadApp } from '..';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
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
  }),
);

export default function(props) {
  const classes = useStyles();
  const [add, setAdd] = useState(false); // 是否正在运行
  const [openMsg, setOpenMsg] = useState(false);
  const [msg, setMsg] = useState({ type: null, text: '' });

  const handleAdd = React.useCallback(async (item: Item) => {
    try {
      let config = await api.get_config();
      if (config.item.some(it => api.compareItems(it, item))) {
        throw '配置已存在！';
      }
      config.item.push(item);
      await api.set_config(config);
      setMsg({ type: 'success', text: '添加成功！' });
      setOpenMsg(true);
      setTimeout(LoadApp, 2000);
      setAdd(false);
    } catch(e) {
      setMsg({ type: 'error', text: `添加失败：${e}` });
      setOpenMsg(true);
    }
  }, []);

  return (
    <Card className={classes.root}>
      <CardHeader
        action={ <IconButton disabled aria-label="settings"> <MoreVertIcon /> </IconButton> }
      />

      <Box padding={5} display="flex" flexDirection="row" flexWrap="wrap" alignItems="flex-start" justifyContent="center">
        <IconButton onClick={() => setAdd(true)}>
          <AddIcon color='disabled' style={{ fontSize: 200 }} />
        </IconButton>
        添加配置
      </Box>

      <CardActions disableSpacing>
        <IconButton className={classes.expand} onClick={() => setAdd(true)}>
            <AddCircleIcon />
        </IconButton>
      </CardActions>
      
      { add &&
        <ItemDialog
          open
          title="添加配置"
          item={{ name: '实例', ws: 'ws://example.com:8080', listen: '127.0.0.1:1066', desc: '' }}
          ok={handleAdd}
          cancel={() => setAdd(false)}
        />
      }

      <Snackbar open={openMsg} autoHideDuration={2000} onClose={() => setOpenMsg(false)}>
        <Alert variant="filled" severity={msg.type}>{msg.text}</Alert>
      </Snackbar>
    </Card>
  );
}
