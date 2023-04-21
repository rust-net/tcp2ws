import React, { useReducer } from 'react';
import TitleBar from './TitleBar';
import ItemCard from './ItemCard';
import Box from '@material-ui/core/Box';
import api, { Config, Item } from '@/api';
import NewItem from './NewItem';

type State = { items: Item[], running: Item[] };

function reducer(state: State, action: State) {
    return { ...state, ...action };
}

export default () => {
    const [state, dispatch] = useReducer(reducer, { items: [], running: [] });

    React.useEffect(() => {
        (async () => {
            let config = await api.config();
            let list = await api.list();
            dispatch({ items: config.item, running: list });
        })();
    }, []);

    return (
        < >
            <TitleBar />
            <Box margin={10} display="flex" flexDirection="row" flexWrap="wrap" alignItems="flex-start" justifyContent="center">
                {
                    state.items.map((item, i) => {
                        return (
                            <ItemCard {...{ item, i: i+1 }} running={state.running.some(it => api.compareItems(it, item))} key={i} />
                        )
                    })
                }
                <NewItem />
            </Box>
        </>
    );
}
