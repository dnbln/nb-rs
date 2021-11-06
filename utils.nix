rec {
  repoTarballFromLink = link: builtins.fetchTarball link;

  repoTarballLink = { user, repo, branch ? "main" }: "https://github.com/${user}/${repo}/archive/${branch}.tar.gz";
  repoTarball = s@{ user, repo, branch ? "main" }: repoTarballFromLink (repoTarballLink s);
  importRepo = s@{ user, repo, branch ? "main" }: import (repoTarball s);


  repoTarballLinkFromRepoLink = { repo, branch ? "main" }: "${repo}/archive/${branch}.tar.gz";
  repoTarballFromRepoLink = s@{ repo, branch ? "main" }: repoTarballFromLink (repoTarballLinkFromRepoLink s);
  importRepoFromLink = link: import (repoTarballFromLink link);

  importRepoFromRepoLink = s@{ repo, branch ? "main" }: import (repoTarballFromRepoLink s);
}
