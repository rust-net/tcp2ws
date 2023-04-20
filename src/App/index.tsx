import React, { useState } from 'react';
import TitleBar from './TitleBar';
import ItemCard from './ItemCard';
import Box from '@material-ui/core/Box';
import api, { Config, Item } from '@/api';

export default () => {
    const [items, setItems] = useState<Item[]>([]);
    const [running, setRunning] = useState<Item[]>([]);

    React.useEffect(() => {
        (async () => {
            let config = await api.config();
            setItems(config.item);
            let list = await api.list();
            setRunning(list);
        })();
    }, []);
    
    return (
        < >
            <TitleBar />
            <Box margin={10} display="flex" flexDirection="row" flexWrap="wrap" alignItems="flex-start" justifyContent="center">
                {
                    items.map((item, i) => {
                        return (
                            <ItemCard {...{ item, i: i+1 }} running={running.some(it => api.compareItems(it, item))} key={i} />
                        )
                    })
                }
            </Box>
        </>
    );
}
