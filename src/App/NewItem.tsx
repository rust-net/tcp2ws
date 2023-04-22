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
import { red } from '@material-ui/core/colors';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import Box from '@material-ui/core/Box';
import AddIcon from '@material-ui/icons/Add';
import AddCircleIcon from '@material-ui/icons/AddCircle';
import PlayCircleFilledWhiteIcon from '@material-ui/icons/PlayCircleFilledWhite';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
import api, { Item } from '@/api';
import Snackbar from '@material-ui/core/Snackbar';
import Alert from '@material-ui/lab/Alert';

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

  const handleClick = async () => {};

  return (
    <Card className={classes.root}>
      <CardHeader
        action={ <IconButton disabled aria-label="settings"> <MoreVertIcon /> </IconButton> }
      />

      <Box padding={5} display="flex" flexDirection="row" flexWrap="wrap" alignItems="flex-start" justifyContent="center">
        <IconButton>
          <AddIcon color='disabled' style={{ fontSize: 200 }} />
        </IconButton>
        添加配置
      </Box>

      <CardActions disableSpacing>
        <IconButton className={classes.expand} onClick={handleClick}>
            <AddCircleIcon />
        </IconButton>
      </CardActions>
    </Card>
  );
}
