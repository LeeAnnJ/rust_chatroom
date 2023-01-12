/*
用户信息相关模块
*/

import axios from './http';
import querystring from 'querystring';

const userApi = {
    getUserByName(params){
        return axios.get('/user/getUserByName',{params:params});
    },
    register(params){
        return axios.post('/user/registor',params); 
        // return axios.post('/user/registor',querystring.stringify(params)); 
    },
    login(params){
        return axios.get('/user/login',{params:params});
    },
    alterPassword(params){
        return axios.post('/user/alterPassword',params);
        // return axios.post('/user/alterPassword',querystring.stringify(params));
    },
    search(params){
        return axios.get('/user/search',{params:params});
    },
    getUserById(params){
        return axios.get('/user/getUserById',{params:params});
    }
};

export default userApi;

