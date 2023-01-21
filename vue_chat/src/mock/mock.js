import Mock from 'mockjs';

Mock.mock('api/user/login/momo/125', function(){ //输出数据
    return Mock.mock({
        "status": 0,
        "data": {
            "id":1,
            "result":true
        }
    })
})
// /api\/friend\/getList.*/
// Mock.mock(/api\/user\/getUserByName.*/,'get',function(){
//     return Mock.mock({
//         "status": 0,
//         "data": {
//             // "users":[]
//             "users":[{
//                 "ID":1,
//                 "PW":"123",
//                 "uName":"momo"
//             }]
//         }
//     })
// })

Mock.mock(/api\/user\/getUserById.*/,'get',function(){
    return Mock.mock({
        "status": 0,
        "data": {
            // "users":[]
            "user":{
                "ID":1,
                "PW":"123",
                "uName":"momo"
            }
        }
    })
})

// Mock.mock('api/user/registor','post', function(){ 
//     return Mock.mock({
//         "status": 0,
//         "data": {
//             "id":4,
//             "result":true
//         }
//     })
// })


Mock.mock('api/friend/getList','get',function(params){
    console.log(params);
    return Mock.mock({
        "status": 0,
        "data": {
            "friends": [{
                "ID": 3,
                "uName": "xixi"
            },{
                "ID": 2,
                "uName": "nono"
            }
            ]
        }
    })
})
Mock.mock(/api\/friend\/getList.*/,'get',function(params){
    return Mock.mock({
        "status": 0,
        "data": {
            "friends": [{
                "ID": 3,
                "uName": "xixi"
            },{
                "ID": 2,
                "uName": "nono"
            }
            ]
        }
    })
})