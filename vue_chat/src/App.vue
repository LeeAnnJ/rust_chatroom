<template>
  <div id="app">
    <nav>
      <router-link to="/Login">Login</router-link> |
      <router-link to="/Register">Register</router-link>  
      <!-- <router-link to="/Room">Room</router-link> -->
    </nav>
    <router-view/>
  </div>
</template>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;

}

nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #fff;

    &.router-link-exact-active {
      color: #f1b6f9;
    }
  }
}

body {
    height: 100%;
    width: 100%;
    background: url("./assets/bg.jpg") no-repeat;
    background-size: cover;
}
</style>
