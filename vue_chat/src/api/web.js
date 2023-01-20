/*
传输层部分模块，路径待处理
*/

import axios from './http';
import querystring from 'querystring';

const webApi = {
    access(params){
        return axios.get(`api/link/${userID}`,{params:params});
    },
    createRoom(params){
        return axios.post(`api/createRoom/${roomName}`,params); 
        // return axios.post(`/createRoom/${roomName}`,querystring.stringify(params)); 
    },
    getRoomList(params){
        return axios.get('api/list',{params:params});
    },
    enterRoom(params){
        return axios.post(`api/enter/${userName}/${roomName}`,params);
        // return axios.post(`/enter/${userName}/${roomName}`,querystring.stringify(params)); 
    },
    exitRoom(params){
        return axios.post(`api/exit/${roomName}`,params);
        // return axios.post(`/exit/${roomName}`,querystring.stringify(params)); 
    }
};

export default webApi;

