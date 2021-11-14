<script lang="ts">
import { tower } from "../../../miner";

import Check from "../Check.svelte";

let localFiles: [string] = [""];

tower.subscribe((t) =>{
  localFiles = t.proofs_saved
})
  
</script>
<main>
  {#if localFiles}
  {#each localFiles as f }
      <Check isTrue={true} title="" description={f}/>
  {/each}
  {/if}
  <!-- <Check isTrue={true} title="CPU Running" description=""/> -->
</main>