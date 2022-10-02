<script lang="ts">
  export let searched = "";
</script>

<div class="search-functions w-[250px]">
  <input
    id="search"
    type="search"
    name="search-function"
    placeholder="Search a function.."
    bind:value={searched}
  />
</div>

<style>
  .search-functions {
    border: 0.5px var(--borders-color) solid;
    font-family: var(--primary-font-family);
    background-color: var(--tertiary-color);
    font-size: 1rem;
    font-weight: 500;
    display: flex;
    padding: 10px;
    height: 60px;
    width: 300px;
    align-items: center;
  }
  .search-functions:hover {
    background-color: var(--primary-color);
  }
  .search-functions p:hover {
    border-bottom: 1px solid var(--secondary-font-color);
  }
  .search-functions p {
    text-decoration: none;
    color: var(--secondary-font-color);
  }

  #search {
    height: 100%;
    width: 100%;
    font-family: var(--primary-font-family);
    font-weight: var(--primary-font-weight);
    border: 1px solid grey;
    border-radius: 10px;
    padding: 10px;
  }
</style>
