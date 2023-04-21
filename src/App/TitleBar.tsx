import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import IconButton from '@material-ui/core/IconButton';
import ExitButton from './ExitButton';
import SvgIcon from '@material-ui/core/SvgIcon';
import Exit from '@material-ui/icons/PowerSettingsNewOutlined';
import CloudDoneIcon from '@material-ui/icons/CloudDone';
import SyncIcon from '@material-ui/icons/Sync';
import { LoadApp } from '@/index';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  header: {
    background: 'lightseagreen',
  },
  menuButton: {
    marginRight: theme.spacing(2),
  },
  title: {
    flexGrow: 1,
  },
}));

export default function ButtonAppBar() {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <AppBar position="static" className={classes.header}>
        <Toolbar>
          <IconButton edge="start" className={classes.menuButton} color="inherit" aria-label="menu">
            <CloudDoneIcon />
          </IconButton>
          <Typography variant="h6" className={classes.title}>
            Tcp2ws
          </Typography>
          <Button onClick={LoadApp} color="inherit"> <SyncIcon></SyncIcon>刷新 </Button>
          <ExitButton> <SvgIcon><Exit /></SvgIcon>关闭应用 </ExitButton>
        </Toolbar>
      </AppBar>
    </div>
  );
}
