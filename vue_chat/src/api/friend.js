/*
好友信息相关模块
*/

import axios from './http';
import querystring from 'querystring';

const friendApi = {
    create(params){
        return axios.post('/friend/create',params); 
        // return axios.post('/friend/create',querystring.stringify(params)); 
    },
    getList(params){
        return axios.get('/friend/getList',{params:params});
    }
};

export default friendApi;

