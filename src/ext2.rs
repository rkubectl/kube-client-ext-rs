use std::collections::BTreeMap;
use std::fmt;

use client::ResourceExt as _;
use k8s::DeploymentGetExt as _;
use k8s::OwnerReferenceExt as _;
use k8s::ReplicaSetGetExt as _;
use k8s::StatefulSetGetExt as _;

use super::*;

/// Async extentions to `kube::Client`
///
#[async_trait::async_trait]
pub trait KubeClientExt2: KubeClientExt {
    /// Get named configmap from a given (or default) namespace
    /// Return `None` if not found`
    ///
    async fn get_configmap_opt(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Option<corev1::ConfigMap>> {
        self.configmaps(namespace).get_opt(name).await
    }

    /// Get named configmap from a given (or default) namespace
    ///
    async fn get_configmap(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<corev1::ConfigMap> {
        self.configmaps(namespace).get(name).await
    }

    /// Get named secret from a given (or default) namespace
    /// Return `None` if not found`
    ///
    async fn get_secret_opt(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Option<corev1::Secret>> {
        self.secrets(namespace).get_opt(name).await
    }

    /// Get named secret from a given (or default) namespace
    ///
    async fn get_secret(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<corev1::Secret> {
        self.secrets(namespace).get(name).await
    }

    /// Get named deployment from a given (or default) namespace
    /// Return `None` if not found
    ///
    async fn get_deployment_opt(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Option<appsv1::Deployment>> {
        self.deployments(namespace).get_opt(name).await
    }

    /// Get named deployment from a given (or default) namespace
    ///
    async fn get_deployment(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<appsv1::Deployment> {
        self.deployments(namespace).get(name).await
    }

    /// Get named statefulset from a given (or default) namespace
    /// Return `None` if not found
    ///
    async fn get_statefulset_opt(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Option<appsv1::StatefulSet>> {
        self.statefulsets(namespace).get_opt(name).await
    }

    /// Get named statefulset from a given (or default) namespace
    ///
    async fn get_statefulset(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<appsv1::StatefulSet> {
        self.statefulsets(namespace).get(name).await
    }

    /// Get named api service
    /// Return `None` if not found
    ///
    async fn get_apiservice_opt(
        &self,
        name: &str,
    ) -> client::Result<Option<apiregistrationv1::APIService>> {
        self.apiservices().get_opt(name).await
    }

    /// Get named api service
    ///
    async fn get_apiservice(&self, name: &str) -> client::Result<apiregistrationv1::APIService> {
        self.apiservices().get(name).await
    }

    /// Get named CRD
    /// Return `None` if not found
    ///
    async fn get_crd_opt(
        &self,
        name: &str,
    ) -> client::Result<Option<apiextensionsv1::CustomResourceDefinition>> {
        self.crds().get_opt(name).await
    }

    /// Get named CRD
    ///
    async fn get_crd(
        &self,
        name: &str,
    ) -> client::Result<apiextensionsv1::CustomResourceDefinition> {
        self.crds().get(name).await
    }

    /// Get owner object from `ownerReference` assuming it is of kind `K`
    ///
    async fn get_owner_k<O, K>(&self, o: &O) -> client::Result<Option<K>>
    where
        O: client::ResourceExt + Sync,
        K: Clone
            + fmt::Debug
            + k8s::openapi::serde::de::DeserializeOwned
            + client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        let dynamic_default = K::DynamicType::default();
        let kind = K::kind(&dynamic_default);
        let namespace = o.namespace();
        if let Some(name) = o
            .owner_references()
            .iter()
            .find(|owner| owner.kind == kind)
            .map(|owner| &owner.name)
        {
            self.namespaced_k(namespace.as_deref()).get_opt(name).await
        } else {
            Ok(None)
        }
    }

    /// List all `Pod`s  in a given (or default) namespace
    ///
    async fn list_pods(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<corev1::Pod>> {
        self.list_k(namespace).await
    }

    /// List all `Deployment`s in a given (or default) namespace
    ///
    async fn list_deployments(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<appsv1::Deployment>> {
        self.list_k(namespace).await
    }

    /// List all `ReplicaSets` in a given (or default) namespace
    ///
    async fn list_replicasets(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<appsv1::ReplicaSet>> {
        self.list_k(namespace).await
    }

    /// List all `Job`s in a given (or default) namespace
    ///
    async fn list_jobs(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<batchv1::Job>> {
        self.list_k(namespace).await
    }

    /// List all `CronJob`s in a given (or default) namespace
    ///
    async fn list_cronjobs(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<batchv1::CronJob>> {
        self.list_k(namespace).await
    }

    /// List all `Secret`s in a given (or default) namespace
    ///
    async fn list_secrets(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<corev1::Secret>> {
        self.list_k(namespace).await
    }

    /// List all `Service`s in a given (or default) namespace
    ///
    async fn list_services(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<corev1::Service>> {
        self.list_k(namespace).await
    }

    /// List all `StatefulSet`s in a given (or default) namespace
    ///
    async fn list_statefulsets(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<appsv1::StatefulSet>> {
        self.list_k(namespace).await
    }

    /// List all `ConfigMap`s in a given (or default) namespace
    ///
    async fn list_configmaps(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<corev1::ConfigMap>> {
        self.list_k(namespace).await
    }

    /// List all `ServiceAccount`s in a given (or default) namespace
    ///
    async fn list_serviceaccounts(
        &self,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Vec<corev1::ServiceAccount>> {
        self.list_k(namespace).await
    }

    /// List namespaced objects of kind `K` in a given (or default) namespace
    ///
    async fn list_k<K>(&self, namespace: impl Into<Option<&str>> + Send) -> client::Result<Vec<K>>
    where
        K: Clone
            + fmt::Debug
            + k8s::openapi::serde::de::DeserializeOwned
            + client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        let lp = self.list_params();
        self.namespaced_k(namespace)
            .list(&lp)
            .await
            .map(|list| list.items)
    }

    /// Get all the pods associated with the deployment
    /// The logic is based on what `kubectl describe` does
    ///
    async fn get_pods_by_deployment_name(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Option<Vec<corev1::Pod>>> {
        // Get the deployment
        let Some(deployment) = self.get_deployment_opt(name, namespace).await? else {
            return Ok(None);
        };
        self.get_pods_by_deployment(&deployment).await
    }

    /// Get all the pods associated with the `deployment`
    /// The logic is based on what `kubectl describe` does
    ///
    async fn get_pods_by_deployment(
        &self,
        deployment: &appsv1::Deployment,
    ) -> client::Result<Option<Vec<corev1::Pod>>> {
        let namespace = deployment.namespace();
        // Get all its replicas
        let mut replicasets = self
            .list_replicasets(namespace.as_deref())
            .await?
            .into_iter()
            .filter(|rs| rs.is_controlled_by(deployment))
            .collect::<Vec<_>>();

        // Find the `NewReplicaSet`
        replicasets.sort_by_key(|rs| rs.creation_timestamp());
        let Some(new) = replicasets
            .iter()
            .find(|rs| match_template_spec_no_hash(rs, deployment))
        else {
            return Ok(None);
        };

        // Find all the Pods controlled by this ReplicaSet
        let pods = self
            .list_pods(namespace.as_deref())
            .await?
            .into_iter()
            .filter(|pod| pod.is_controlled_by(new))
            .collect();

        Ok(Some(pods))
    }

    /// Get all the pods controlled by a given statefulset
    async fn get_pod_by_statefulset(
        &self,
        statefulset: &appsv1::StatefulSet,
    ) -> client::Result<Option<Vec<corev1::Pod>>> {
        let namespace = statefulset.namespace();
        let pods = if let Some(revision) = statefulset.current_revision() {
            let controller_revision = format!(
                "{}={}",
                k8s::label::CONTROLLER_REVISION_HASH_LABEL_KEY,
                revision
            );
            let lp = self.list_params().labels(&controller_revision);
            self.pods(namespace.as_deref()).list(&lp).await?.items
        } else {
            vec![]
        };
        Ok(Some(pods))
    }

    async fn get_pods_by_statefulset_name(
        &self,
        name: &str,
        namespace: impl Into<Option<&str>> + Send,
    ) -> client::Result<Option<Vec<corev1::Pod>>> {
        let Some(statefulset) = self.get_statefulset_opt(name, namespace).await? else {
            return Ok(None);
        };
        self.get_pod_by_statefulset(&statefulset).await
    }
}

impl KubeClientExt2 for client::Client {}

fn match_template_spec_no_hash(rs: &appsv1::ReplicaSet, deployment: &appsv1::Deployment) -> bool {
    let rs_template = rs.template().map(remove_hash);
    let deployment_template = deployment.template().map(remove_hash);
    rs_template == deployment_template
}

fn remove_hash(template: &corev1::PodTemplateSpec) -> corev1::PodTemplateSpec {
    let mut template = template.clone();
    if let Some(labels) = labels_mut(&mut template) {
        labels.remove(k8s::label::DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY);
    }
    template
}

fn labels_mut(template: &mut corev1::PodTemplateSpec) -> Option<&mut BTreeMap<String, String>> {
    template.metadata.as_mut()?.labels.as_mut()
}
