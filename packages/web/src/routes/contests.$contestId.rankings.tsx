import { useTranslation } from '@broccoli/web-sdk/i18n';
import { Slot } from '@broccoli/web-sdk/slot';
import { Button } from '@broccoli/web-sdk/ui';
import { BarChart3 } from 'lucide-react';
import { Link, useParams, useSearchParams } from 'react-router';

import { PageLayout } from '@/components/PageLayout';
import { useContestInfo } from '@/features/contest/hooks/use-contest-info';
import { supportsPublicScoreboard } from '@/features/rankings/public-scoreboard';

export default function ContestRankingPage() {
  const { t } = useTranslation();
  const { contestId } = useParams();
  const [searchParams] = useSearchParams();
  const publicView = searchParams.get('view') === 'public';
  const { contest } = useContestInfo(Number(contestId));
  const supported = supportsPublicScoreboard(contest?.contest_type);

  return (
    <PageLayout
      pageId="ranking"
      icon={<BarChart3 className="h-6 w-6 text-sidebar-primary" />}
      title={t(publicView ? 'ranking.publicTitle' : 'ranking.title')}
      contentClassName="flex flex-col gap-6"
    >
      {publicView && (
        <div className="flex flex-wrap items-center justify-between gap-3 rounded-lg border p-4">
          <p className="text-sm text-muted-foreground">
            {t('ranking.publicNotice')}
          </p>
          <Button asChild variant="outline" size="sm">
            <Link to={`/contests/${contestId}/rankings`}>
              {t('ranking.backToRanking')}
            </Link>
          </Button>
        </div>
      )}
      {(!publicView || supported) && (
        <Slot name="ranking.header" as="div" slotProps={{ publicView }} />
      )}
      {publicView && !supported ? (
        <div className="rounded-lg border border-dashed p-8 text-center text-sm text-muted-foreground">
          {contest ? t('ranking.publicUnsupported') : t('ranking.empty')}
        </div>
      ) : (
        <Slot
          key={publicView ? 'public' : 'default'}
          name="ranking.content"
          as="div"
          className="w-full"
          slotProps={{ publicView }}
        >
          <div className="rounded-lg border border-dashed p-8 text-center text-sm text-muted-foreground">
            {t('ranking.empty')}
          </div>
        </Slot>
      )}
    </PageLayout>
  );
}
