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
import CloudDoneIcon from '@material-ui/icons/CloudDone';
import CloudOffIcon from '@material-ui/icons/CloudOff';
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

export default function RecipeReviewCard(props: { item: Item, i: number, running: boolean }) {
  const classes = useStyles();
  const [expanded, setExpanded] = useState(props.running); // 是否正在运行
  const [openMsg, setOpenMsg] = useState(false);
  const [msg, setMsg] = useState({ type: null, text: '' });

  useEffect(() => setExpanded(props.running), [props.running]);
  useEffect(() => {
    if (openMsg) {
        console.log('设置');
        let task = setTimeout(() => setOpenMsg(false), 2000);
        return () => {
            console.log('取消');
            clearTimeout(task);
        }
    }
  }, [openMsg, msg]);

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
        action={ <IconButton aria-label="settings"> <MoreVertIcon /> </IconButton> }
        title={props.item.name}
        subheader={props.item.ws}
      />

      <Box padding={5} display="flex" flexDirection="row" flexWrap="wrap" alignItems="flex-start" justifyContent="center">
        {expanded ?  <CloudDoneIcon style={{ fontSize: 200, color: 'greenyellow' }} /> : <CloudOffIcon color='disabled' style={{ fontSize: 200 }} />}
        {props.item.listen}
      </Box>

      <CardContent>
        <Typography variant="body2" color="textSecondary" component="div">
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

      <Snackbar open={openMsg}>
        <Alert variant="filled" severity={msg.type}>{ msg.text }</Alert>
      </Snackbar>
    </Card>
  );
}
