/*
聊天记录相关模块
*/

import axios from './http';
import querystring from 'querystring';

const mesApi = {
    addmessage(params){
        return axios.post('api/message/addmessage',params); 
        // return axios.post('/message/addmessage',querystring.stringify(params)); 
    },
    setRead(params){
        return axios.post('api/message/setRead',params); 
        // return axios.post('/message/setRead',querystring.stringify(params)); 
    },
    getList(params){
        return axios.get('api/message/getList',{params:params});
    }
};

export default mesApi;

