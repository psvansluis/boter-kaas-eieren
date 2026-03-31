<script lang="ts">
  const {
    actie,
  }: {
    actie: {
      label: string;
      onclick: () => void;
      disabled: boolean;
      sneltoets: string;
    };
  } = $props();

  const ariaLabel = $derived(
    `${actie.label} (druk ${actie.sneltoets.toUpperCase()})`,
  );

  const klasse = $derived(actie.disabled ? "onspeelbaar" : "speelbaar");
</script>

<button
  class="actieknop {klasse}"
  aria-label={ariaLabel}
  aria-keyshortcuts={actie.sneltoets}
  aria-disabled={actie.disabled}
  onclick={(e) => {
    if (actie.disabled) {
      e.preventDefault();
      return;
    }
    actie.onclick();
  }}
>
  {actie.label}
  <span class="sneltoets">{actie.sneltoets.toUpperCase()}</span>
</button>
