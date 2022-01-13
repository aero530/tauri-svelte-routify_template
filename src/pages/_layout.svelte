<script>
    import {url} from '@roxi/routify'
	import "carbon-components-svelte/css/white.css";
	import {
        Header,
        SideNav,
        SideNavItems,
        SideNavMenu,
        SideNavMenuItem,
        SideNavLink,
        SideNavDivider,
        SkipToContent,
        Content,
        Grid,
        Row,
        Column,
	} from "carbon-components-svelte";
    import Fade16 from "carbon-icons-svelte/lib/Fade16";

	let isSideNavOpen = false;
</script>

<Header company="companyname" platformName="Carbon Svelte" bind:isSideNavOpen>
    <div slot="skip-to-content">
      <SkipToContent />
    </div>
</Header>
  
<SideNav bind:isOpen={isSideNavOpen} rail>
    <SideNavItems>
        <SideNavLink icon={Fade16} href={$url("./index")} text="Dashboard"/>
        <SideNavLink icon={Fade16} href={$url("./Settings")} text="Settings"/>
        <SideNavDivider />
      
        <SideNavMenu text="Menu">
            <SideNavMenuItem href="/" text="Link 1" />
            <SideNavMenuItem href="/" text="Link 2" />
            <SideNavMenuItem href="/" text="Link 3" />
          </SideNavMenu>
          <SideNavLink text="Link 4" />
    </SideNavItems>
</SideNav>
<main>
<Content>
    <Grid>
      <Row>
        <Column>
            <slot />
        </Column>
      </Row>
    </Grid>
</Content>

</main>


<style>
main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
}

@media (min-width: 640px) {
    main {
        max-width: none;
    }
}
</style>