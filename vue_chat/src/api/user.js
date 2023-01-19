/*
用户信息相关模块
*/

import axios from './http';
import querystring from 'querystring';

const userApi = {
    getUserByName(params){
        return axios.get('api/user/getUserByName',{params:params});
    },
    register(params){
        return axios.post('api/user/registor',params); 
        // return axios.post('api/user/registor',querystring.stringify(params)); 
    },
    login(username,password){
        return axios.get(`api/user/login/${username}/${password}`);
    },
    alterPassword(params){
        return axios.post('api/user/alterPassword',params);
        // return axios.post('/user/alterPassword',querystring.stringify(params));
    },
    search(params){
        return axios.get('api/user/search',{params:params});
    },
    getUserById(params){
        return axios.get('api/user/getUserById',{params:params});
    }
};

export default userApi;

